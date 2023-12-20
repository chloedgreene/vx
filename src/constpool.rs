use binread::{io::Cursor, BinReaderExt};
use std::io::Read;

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum ConstantPoolTags {
    Class(u16), //name index
    Fieldref(u16, u16),
    Methodref(u16, u16), //class index, name and type index
    InterfaceMethodref(u16, u16),
    String(u16),
    Integer,
    Float,
    Long,
    Double,
    NameAndType(u16, u16), //name_index, decriptor index
    Utf8(u16, String),     // length, bytes
    MethodHandle,
    MethodType,
    InvokeDynamic,
}

pub fn get_constant_pool(
    constant_pool_count: u16,
    reader: &mut Cursor<Vec<u8>>,
) -> Vec<ConstantPoolTags> {
    {
        let mut working_pool = vec![];
        for _i in 0..constant_pool_count - 1 {
            let tag: u8 = reader.read_be().unwrap();
            let info: ConstantPoolTags = match tag {
                1 => {
                    let length: u16 = reader.read_be::<u16>().unwrap();
                    let mut bytes = vec![0u8; length as usize];
                    reader.read_exact(&mut bytes).unwrap();
                    let text = String::from_utf8(bytes).unwrap();

                    ConstantPoolTags::Utf8(length, text)
                }

                8 => {
                    let string_index: u16 = reader.read_be::<u16>().unwrap();
                    ConstantPoolTags::String(string_index)
                }

                9 | 10 | 11 => {
                    // all have the same data, just diferent names
                    let class_index: u16 = reader.read_be::<u16>().unwrap();
                    let name_and_type_index: u16 = reader.read_be::<u16>().unwrap();
                    match tag {
                        //recalculate it into the right enum type
                        9 => ConstantPoolTags::Fieldref(class_index, name_and_type_index),
                        10 => ConstantPoolTags::Methodref(class_index, name_and_type_index),
                        11 => {
                            ConstantPoolTags::InterfaceMethodref(class_index, name_and_type_index)
                        }
                        _ => {
                            panic!("Somehow the tag changed in 3 insturctions, uhhhh, maybe a cosmic ray did a bitflip?")
                        }
                    }
                }

                7 => {
                    let name_index: u16 = reader.read_be::<u16>().unwrap();
                    ConstantPoolTags::Class(name_index)
                }

                12 => {
                    let name_index: u16 = reader.read_be::<u16>().unwrap();
                    let descriptor_index: u16 = reader.read_be::<u16>().unwrap();
                    ConstantPoolTags::NameAndType(name_index, descriptor_index)
                }

                _ => {
                    todo!("Impliment tag: {}", tag)
                }
            };

            working_pool.push(info);
        }
        return working_pool;
    };
}
