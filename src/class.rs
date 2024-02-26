use crate::{method::MethodInfo, constpool::{ConstantPoolTags, get_constant_pool}, stubs::{Interfaces, Fields}, attributes::{Attribute, get_atributes}};

use binread::{io::Cursor, BinReaderExt};

#[derive(Debug, Clone)]
pub struct class{
    pub version: (u16,u16),
    pub constant_pool: Vec<ConstantPoolTags>,
    pub accessflag : u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces : Vec<Interfaces>,
    pub fields: Vec<Fields>,
    pub methods : Vec<MethodInfo>,
    pub classattributes: Vec<Attribute>
}

pub fn load_classes(path_list:&[String]) ->Vec<class>{

    let mut classes:Vec<class> = vec![];
    let mut cp:Vec<ConstantPoolTags> = vec![];
    for potention_class in path_list{ // load every class in
        let classfile = std::fs::read(potention_class).unwrap();
        let mut reader = Cursor::new(classfile.clone());
        if reader.read_be::<u32>().unwrap() != 0xCAFEBABE {
            // checks magic id of class files
            panic!("Magic is not valid for file: {}",potention_class)
        }

        //after we check the magic value is valid, we can start loading it inot the class list!
        classes.push(class{
            version : {
                let minor = reader.read_be::<u16>().unwrap();
                let major = reader.read_be::<u16>().unwrap();
                (major, minor)
            },
            constant_pool : {
                let constant_pool_count = reader.read_be::<u16>().unwrap();
                cp = get_constant_pool(constant_pool_count, &mut reader);
                cp.clone()
            },
            accessflag: {
                reader.read_be::<u16>().unwrap()
            },
            this_class : {
                reader.read_be::<u16>().unwrap()
            },
            super_class: {
                reader.read_be::<u16>().unwrap()
            },
            interfaces : {
                let interface_count = reader.read_be::<u16>().unwrap();
                if interface_count != 0 {panic!("im not making a ful java implimentations, so we skip these, i gotta do these some day")}
                vec![]
            },
            fields : {
                let field_count = reader.read_be::<u16>().unwrap();
                if field_count != 0 {
                    todo!("no field implimentation")
                }
                vec![]
            },
            methods : {
                let method_count = reader.read_be::<u16>().unwrap();
                let mut methodbuffer: Vec<MethodInfo> = vec![];

                for _i  in  0..method_count{
                    let methodaccess_flags = reader.read_be::<u16>().unwrap();
                    let name_index = reader.read_be::<u16>().unwrap();
                    let descriptor_index = reader.read_be::<u16>().unwrap();
                    let attributes_count = reader.read_be::<u16>().unwrap();
                    let cp_buffer = cp.clone();
                    let attributes: Vec<Attribute> = get_atributes(attributes_count, &mut reader, &cp_buffer);
                    methodbuffer.push(MethodInfo {
                        access_flag: methodaccess_flags,
                        name_index,
                        descriptor_index,
                        attributes_count,
                        attributes,
                    })
                }

                methodbuffer
            },
            classattributes :{
                let attribute_count = reader.read_be::<u16>().unwrap();
                get_atributes(attribute_count, &mut reader, &cp)
            }
        });
    
        
        if reader.position() == classfile.len() as u64 { // we need a true for this statement because reading the position actually changes internal varubles and we need that to heappen
        } else {
            panic!("parsed done not at end of file")
        }
    };

    classes

}

