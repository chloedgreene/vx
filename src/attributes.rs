
use binread::{BinRead, io::Cursor, BinReaderExt};
use std::{task::Context, io::Read};

use crate::{constpool::ConstantPoolTags, attributes};



#[derive(Debug)]
pub struct attribute{
    pub attribute_name_index: u16,
    pub attribute_length : u32,
    pub info:attribute_info

}

#[derive(Debug)]
pub enum attribute_info{
    LineNumberTable(u16,Vec<(u16,u16)>),
    Code(u16,u16,u32,Vec<u8>,u16,u16,Vec<attribute>),
    SourceFile(u16),
    DUMMY //TODO: REMOVE
}

pub fn get_atributes(attributes_count:u16, reader: &mut Cursor<Vec<u8>>, constant_pool: &Vec<ConstantPoolTags>) -> Vec<attribute>{

    let mut attributes:Vec<attribute> = vec![];
    for i in 0..attributes_count{
        let attribute_name_index = reader.read_be::<u16>().unwrap();
        let attribute_length = reader.read_be::<u32>().unwrap();
        let attribute_type = &constant_pool[attribute_name_index as usize -1];
        
        println!("Printing Attributes: {:?}",attribute_type);

        let info:attributes::attribute_info = match attribute_type {

            ConstantPoolTags::Utf8(_, attrupt) => {

                let att = match attrupt.as_str() {
                    "LineNumberTable" => { // attribute_length
                        let lntl = reader.read_be::<u16>().unwrap(); //line_number_table_length
                        
                        let mut lnt:Vec<(u16,u16)> = vec![]; //line number tbale
                        for i in 0..lntl{
                            let spc = reader.read_be::<u16>().unwrap(); //starting index of code
                            let ln = reader.read_be::<u16>().unwrap(); //liine number
                            lnt.push((spc,ln))
                        }
                        attributes::attribute_info::LineNumberTable(lntl, lnt)
                    }

                    "Code" =>{
                        let max_stack = reader.read_be::<u16>().unwrap();
                        let max_locals = reader.read_be::<u16>().unwrap();
                        let code_length = reader.read_be::<u32>().unwrap();
                        let code = {
                            let mut bytes = vec![0u8; code_length as usize];
                            reader.read_exact(&mut bytes).unwrap();
                            bytes
                        };
                        let exception_table_length = reader.read_be::<u16>().unwrap();
                        for i in 0..exception_table_length{
                            panic!("Cant parse exceptions yet")
                        }
                        let attribues_count = reader.read_be::<u16>().unwrap();
                        let atatributes = get_atributes(attributes_count, reader, constant_pool);

                        attributes::attribute_info::Code(max_stack, max_locals, code_length, code, exception_table_length, attribues_count, atatributes)
                    }
                    
                    "SourceFile" => {
                        let sourcefile_index = reader.read_be::<u16>().unwrap();
                        attributes::attribute_info::SourceFile(sourcefile_index)
                    }
                    
                    _ => { todo!("Add: {}",attrupt)}
                };
                att

            }

            _ => { todo!("Not utf8")}
            
        };
        
        let atty = attribute { attribute_name_index, attribute_length, info };
        println!("Attribute Inter Info : {:?}",&atty);
        attributes.push(atty);
    }
    attributes

    
}