use std::{task::Context, io::Read};

use binread::{BinRead, io::Cursor, BinReaderExt};
use constpool::ConstantPoolTags;

use crate::{constpool::{get_constant_pool}, method::method_info, attributes::{attribute, get_atributes}, javafunctionsrs::execute_internel_function};

mod constpool;
mod method;
mod attributes;
mod javafunctionsrs;

const HELLOWORLD: &[u8; 417] = include_bytes!("../code/Hello.class");

enum ClassAccessFlags{
    PUBLIC 	    =  0x0001,
    FINAL 	    =  0x0010,
    SUPER 	    =  0x0020,
    INTERFACE  = 	0x0200,
    ABSTRACT 	 = 0x0400,
    SYNTHETIC  = 	0x1000,
    ANNOTATION  = 	0x2000,
    ENUM 	    =  0x4000,
}

#[derive(Debug,Clone)]
pub enum Varubals{
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Float,
    Reference(ReferenceType),
    ReturnAddress,
    Class(String,String,String),
    String(String)
}
#[derive(Debug,Copy,Clone)]
pub enum ReferenceType{
    Constpool(u16)
}

fn main() {
    
    
    let mut reader = Cursor::new(HELLOWORLD.to_vec());
    if reader.read_be::<u32>().unwrap() != 0xCAFEBABE{ // checks magic id of class files
        panic!("Magic is not valid")
    }

    let (major,minor) = {
        let minor = reader.read_be::<u16>().unwrap();
        let major = reader.read_be::<u16>().unwrap();
        (major,minor)
    };

    println!("Version: {}.{}",major,minor);

    let constant_pool_count = reader.read_be::<u16>().unwrap();
    let constant_pool:Vec<ConstantPoolTags> = get_constant_pool(constant_pool_count,&mut reader);

    {
        println!("Number of Constants: {}",constant_pool_count);
        let mut index = 1;
        for con in &constant_pool {
            println!("{} | {:?}",index,con);
            index+=1;
        }
    }

    let accessflag = reader.read_be::<u16>().unwrap();
    println!("Class Access flag: {:#02x}", accessflag);


    let this_class = reader.read_be::<u16>().unwrap();
    println!("This Class: {}",this_class);
    
    let super_class = reader.read_be::<u16>().unwrap();
    println!("Super Class: {}",super_class);

    let interface_count = reader.read_be::<u16>().unwrap();
    println!("Interface count : {}",interface_count);
    if interface_count != 0 {
        todo!("no interfaces implimentation")
    }

    let field_count = reader.read_be::<u16>().unwrap();
    println!("Field count : {}",field_count);
    if field_count != 0 {
        todo!("no field implimentation")
    }

    let method_count = reader.read_be::<u16>().unwrap();
    println!("Method count : {}",method_count);
    let mut methods:Vec<method_info> = vec![];

    for i in 0..method_count{
        let methodaccess_flags = reader.read_be::<u16>().unwrap();
        let name_index = reader.read_be::<u16>().unwrap();
        let descriptor_index = reader.read_be::<u16>().unwrap();
        let attributes_count = reader.read_be::<u16>().unwrap();

        println!("Method Name{:?}", &constant_pool[name_index as usize]);
        println!("Descriptor {:?}", &constant_pool[descriptor_index as usize]);
        println!("Attributes Count {}", attributes_count);
        println!("Method Access flag: {:#02x}", methodaccess_flags);


        let attributes:Vec<attribute> = get_atributes(attributes_count, &mut reader, &constant_pool);

        methods.push(method_info{
            access_flag: methodaccess_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes
        })
        
    }

    let attribute_count = reader.read_be::<u16>().unwrap();
    let classattributes = get_atributes(attribute_count, &mut reader, &constant_pool);

    if reader.position() == HELLOWORLD.len() as u64{
        println!("Finished Parseing")
    }else {
        panic!("parsed done not at end of file")
    }

    ////////////////////////////////////////////////////////////////////
    /// EXECUTE CODE
    println!("////////////////////////////////////////////////////////////////////");
    ////////////////////////////////////////////////////////////////////
    

    for m in methods{
        println!("{:?}",m);
        for a in m.attributes{

            match a.info {
                attributes::attribute_info::LineNumberTable(_, _) => todo!(),
                attributes::attribute_info::Code(_, _, _, code, _, _, sourceLines) => {

                    println!("Bytecode: {:?}",&code);
                    let length = code.len();
                    let mut bytecode = Cursor::new(code);
                    
                    let mut local_varuble_array:Vec<Varubals> = vec![];
                    local_varuble_array.push(Varubals::Reference(ReferenceType::Constpool(this_class)));

                    let mut operand_stack:Vec<Varubals> = vec![];
                    
                    for i in 0..length{
                        println!("Doing insturction: {}",i);
                        let opcode:u8 = bytecode.read_be().unwrap();
                        match opcode {

                            18 =>{
                                let index:u8 = bytecode.read_be().unwrap();
                                let value = &constant_pool[index as usize -1 ];

                                match value {
                                    ConstantPoolTags::String(utf8_index) => {
                                        if let ConstantPoolTags::Utf8(_, Data) = &constant_pool[*utf8_index as usize -1] {
                                            operand_stack.push(Varubals::String(Data.to_string()))
                                        }
                                    }

                                    _ => {panic!("Loading value not implimented: {:?}",value)}
                                    
                                }

                            }
                            
                            178 => {
                                let mut class_name_final = "";
                                let mut name_final = "";
                                let mut type_final = "";

                                let index:u16 = bytecode.read_be().unwrap();
                                let fieldref = &constant_pool[index as usize -1 ];
                                if let ConstantPoolTags::Fieldref(a,b ) = fieldref{
                                    let class_index = &constant_pool[*a as usize -1 ];
                                    if let ConstantPoolTags::Class(u) = class_index{
                                        let utf8 = &constant_pool[*u as usize -1 ];
                                        if let ConstantPoolTags::Utf8(_, class_name) = utf8{
                                            println!("Class Name: {}",class_name);
                                            class_name_final = class_name;
                                        }
                                    
                                    }





                                    let name_and_type_index = &constant_pool[*b as usize -1 ];
                                    if let ConstantPoolTags::NameAndType(u,j) = name_and_type_index{
                                        let utf81 = &constant_pool[*u as usize -1 ];
                                        let utf82 = &constant_pool[*j as usize -1 ];

                                        

                                        if let ConstantPoolTags::Utf8(_, class_name) = utf81{
                                            println!("Method Name: {}",class_name);
                                            name_final = class_name;
                                        }
                                        if let ConstantPoolTags::Utf8(_, class_name) = utf82{
                                            println!("Method Type: {}",class_name);
                                            type_final = class_name;
                                        }

                                    }





                                }

                                operand_stack.push(Varubals::Class(class_name_final.to_string(), name_final.to_string(), type_final.to_string()));


                            }
                            177 =>{
                                println!("Returning function back up call stack");
                                break;
                            }
                            183 | 182 =>{

                                let index:u16 = bytecode.read_be().unwrap();
                                let methodred = &constant_pool[index as usize -1 ];
                                match methodred {
                                    ConstantPoolTags::Methodref(a, b) => {
                                        let mut class_name_final = "";
                                        let class_index = &constant_pool[*a as usize -1 ];
                                        if let ConstantPoolTags::Class(u) = class_index{
                                            let utf8 = &constant_pool[*u as usize -1 ];
                                            if let ConstantPoolTags::Utf8(_, class_name) = utf8{
                                                println!("Class Name: {}",class_name);
                                                class_name_final = class_name;
                                            }
                                        
                                        }
                                        let mut name_final = "";
                                        let mut type_final = "";

                                        let name_and_type_index = &constant_pool[*b as usize -1 ];
                                        if let ConstantPoolTags::NameAndType(u,j) = name_and_type_index{
                                            let utf81 = &constant_pool[*u as usize -1 ];
                                            let utf82 = &constant_pool[*j as usize -1 ];

                                            

                                            if let ConstantPoolTags::Utf8(_, class_name) = utf81{
                                                println!("Method Name: {}",class_name);
                                                name_final = class_name;
                                            }
                                            if let ConstantPoolTags::Utf8(_, class_name) = utf82{
                                                println!("Method Type: {}",class_name);
                                                type_final = class_name;
                                            }

                                        }

                                        execute_internel_function(class_name_final,name_final,type_final,&mut operand_stack);


                                    },
                                    _ => panic!("Expected Method Ref")
                                }

                            },
                            42 =>{
                                operand_stack.push(local_varuble_array[0].clone());
                            }
                            _ => {panic!("Unknown opcode: {}",opcode)}
                        }
                    }



                },
                attributes::attribute_info::SourceFile(_) => todo!(),
                attributes::attribute_info::DUMMY => todo!(),
            }

        }
    }




}  
