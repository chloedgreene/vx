

use std::{env, process::exit};

use binread::{io::Cursor, BinReaderExt};

use constpool::ConstantPoolTags;



use log::{info, trace};

use crate::{
    attributes::{Attribute, get_atributes},
    constpool::get_constant_pool,
    glocalfunctionsrs::execute_internel_function,
    method::MethodInfo,
};

mod attributes;
mod constpool;
mod glocalfunctionsrs;
mod method;
mod class;

#[allow(dead_code)]
enum ClassAccessFlags {
    PUBLIC = 0x0001,
    FINAL = 0x0010,
    SUPER = 0x0020,
    INTERFACE = 0x0200,
    ABSTRACT = 0x0400,
    SYNTHETIC = 0x1000,
    ANNOTATION = 0x2000,
    ENUM = 0x4000,
}

#[derive(Debug, Clone)]
pub enum Varubals { // stfu im dyslexic, im not changing the spelling mistake
    Boolean,
    Byte,
    Char,
    Short,
    Int(i32),
    Float,
    Reference(ReferenceType),
    ReturnAddress,
    Class(String, String, String),
    String(String),
    Null // EWWWWWWWWW NULL, this is why i use rust, null ikey
}
#[derive(Debug, Copy, Clone)]
pub enum ReferenceType {
    Constpool(u16),
}

/////////////////////////////////////////////
/// HEY THIS IS DEPRECATED, THIS IS OLD CODE, DONT GRADE ME ON THIS LMFAO
/////////////////////////////////////////////
fn omain() {

    let args: Vec<String> = env::args().collect();
    env_logger::builder()
    .init();

    let path = match args.len() {

        1 => {println!("Please Specify A Class File"); exit(1);}
        2 => {&args[1]}
        3 => {println!("Please Specify One 1 file"); exit(1);}

        _ => {panic!("Idk how to respond to : {}",args.len())}
        
    };

    let classfile = std::fs::read(path).unwrap();

    let mut reader = Cursor::new(classfile.clone());
    if reader.read_be::<u32>().unwrap() != 0xCAFEBABE {
        // checks magic id of class files
        panic!("Magic is not valid")
    }

    let (major, minor) = {
        let minor = reader.read_be::<u16>().unwrap();
        let major = reader.read_be::<u16>().unwrap();
        (major, minor)
    };
    info!(target: "class","Version: {}.{}", major, minor);

    let constant_pool_count = reader.read_be::<u16>().unwrap();
    let constant_pool: Vec<ConstantPoolTags> = get_constant_pool(constant_pool_count, &mut reader);

    {
        info!(target: "class","Number of Constants: {}", constant_pool_count);
        let mut index = 1;
        for con in &constant_pool {
        trace!(target: "class_constant","{} | {:?}", index, con);
            index += 1;
        }
    }

    let accessflag = reader.read_be::<u16>().unwrap();
    info!(target: "class","Class Access flag: {:#02x}", accessflag);

    let this_class = reader.read_be::<u16>().unwrap();
    info!(target: "class","This Class: {}", this_class);

    let super_class = reader.read_be::<u16>().unwrap();
    info!(target: "class","Super Class: {}", super_class);

    let interface_count = reader.read_be::<u16>().unwrap();
    info!(target: "class","Interface count : {}", interface_count);
    if interface_count != 0 {
        todo!("no interfaces implimentation")
    }

    let field_count = reader.read_be::<u16>().unwrap();
    info!(target: "class","Field count : {}", field_count);
    if field_count != 0 {
        todo!("no field implimentation")
    }

    let method_count = reader.read_be::<u16>().unwrap();
    info!(target: "class","Method count : {}", method_count);
    let mut methods: Vec<MethodInfo> = vec![];

    for _i in 0..method_count {
        let methodaccess_flags = reader.read_be::<u16>().unwrap();
        let name_index = reader.read_be::<u16>().unwrap();
        let descriptor_index = reader.read_be::<u16>().unwrap();
        let attributes_count = reader.read_be::<u16>().unwrap();

        trace!(target: "method","Method Name{:?}", &constant_pool[name_index as usize]);
        trace!(target: "method","Descriptor {:?}", &constant_pool[descriptor_index as usize]);
        trace!(target: "method","Attributes Count {}", attributes_count);
        trace!(target: "method","Method Access flag: {:#02x}", methodaccess_flags);

        info!(target: "Getting Attribute","Method Access flag: {:#02x}", methodaccess_flags);

        let attributes: Vec<Attribute> =
            get_atributes(attributes_count, &mut reader, &constant_pool);

        methods.push(MethodInfo {
            access_flag: methodaccess_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }

    let attribute_count = reader.read_be::<u16>().unwrap();
    let _classattributes = get_atributes(attribute_count, &mut reader, &constant_pool);

    if reader.position() == classfile.len() as u64 {
        info!(target: "class","Finished Parseing")
    } else {
        panic!("parsed done not at end of file")
    }


    for m in methods {
        let method_name = &constant_pool[m.name_index as usize -1];

        if let ConstantPoolTags::Utf8(_,data) = method_name{
            match data.to_owned().as_str() {
                "<init>" | "main" =>{
                    //do nothing
                }
                _ =>{ continue;}
            }

        }

        info!(target: "exec","{:?}", m);
        for a in m.attributes {
            match a.info {
                attributes::AttributeInfo::LineNumberTable(_, _) => todo!(),
                attributes::AttributeInfo::Code(_, _, _, code, _, _, _source_lines) => {
                    info!(target: "exec","Bytecode: {:?}", &code);
                    let length = code.len();
                    let mut bytecode = Cursor::new(code);

                    let mut local_varuble_array: Vec<Varubals> = vec![Varubals::Null; 256];
                    local_varuble_array
                        .push(Varubals::Reference(ReferenceType::Constpool(this_class)));

                    let mut operand_stack: Vec<Varubals> = vec![];


                    for i in 0..length {
                        trace!(target: "exec","Doing insturction: {}", i);
                        let opcode: u8 = bytecode.read_be().unwrap();
                        match opcode {
                            18 => {
                                let index: u8 = bytecode.read_be().unwrap();
                                let value = &constant_pool[index as usize - 1];

                                match value {
                                    ConstantPoolTags::String(utf8_index) => {
                                        if let ConstantPoolTags::Utf8(_, data) =
                                            &constant_pool[*utf8_index as usize - 1]
                                        {
                                            operand_stack.push(Varubals::String(data.to_string()))
                                        }
                                    }

                                    _ => {
                                        panic!("Loading value not implimented: {:?}", value)
                                    }
                                }
                            }

                            178 => {
                                let mut class_name_final = "";
                                let mut name_final = "";
                                let mut type_final = "";

                                let index: u16 = bytecode.read_be().unwrap();
                                let fieldref = &constant_pool[index as usize - 1];
                                if let ConstantPoolTags::Fieldref(a, b) = fieldref {
                                    let class_index = &constant_pool[*a as usize - 1];
                                    if let ConstantPoolTags::Class(u) = class_index {
                                        let utf8 = &constant_pool[*u as usize - 1];
                                        if let ConstantPoolTags::Utf8(_, class_name) = utf8 {
                                            trace!(target: "exec","Class Name: {}", class_name);
                                            class_name_final = class_name;
                                        }
                                    }

                                    let name_and_type_index = &constant_pool[*b as usize - 1];
                                    if let ConstantPoolTags::NameAndType(u, j) = name_and_type_index
                                    {
                                        let utf81 = &constant_pool[*u as usize - 1];
                                        let utf82 = &constant_pool[*j as usize - 1];

                                        if let ConstantPoolTags::Utf8(_, class_name) = utf81 {
                                            trace!(target: "exec","Method Name: {}", class_name);
                                            name_final = class_name;
                                        }
                                        if let ConstantPoolTags::Utf8(_, class_name) = utf82 {
                                            trace!(target: "exec","Method Type: {}", class_name);
                                            type_final = class_name;
                                        }
                                    }
                                }

                                operand_stack.push(Varubals::Class(
                                    class_name_final.to_string(),
                                    name_final.to_string(),
                                    type_final.to_string(),
                                ));
                            }
                            177 => {
                                trace!(target: "class","Returning function back up call stack");
                                break;
                            }
                            183 | 182 => {
                                let index: u16 = bytecode.read_be().unwrap();
                                let methodred = &constant_pool[index as usize - 1];
                                match methodred {
                                    ConstantPoolTags::Methodref(a, b) => {
                                        let mut class_name_final = "";
                                        let class_index = &constant_pool[*a as usize - 1];
                                        if let ConstantPoolTags::Class(u) = class_index {
                                            let utf8 = &constant_pool[*u as usize - 1];
                                            if let ConstantPoolTags::Utf8(_, class_name) = utf8 {
                                                info!(target: "exec","Class Name: {}", class_name);
                                                class_name_final = class_name;
                                            }
                                        }
                                        let mut name_final = "";
                                        let mut type_final = "";

                                        let name_and_type_index = &constant_pool[*b as usize - 1];
                                        if let ConstantPoolTags::NameAndType(u, j) =
                                            name_and_type_index
                                        {
                                            let utf81 = &constant_pool[*u as usize - 1];
                                            let utf82 = &constant_pool[*j as usize - 1];

                                            if let ConstantPoolTags::Utf8(_, class_name) = utf81 {
                                                trace!(target: "exec","Method Name: {}", class_name);
                                                name_final = class_name;
                                            }
                                            if let ConstantPoolTags::Utf8(_, class_name) = utf82 {
                                                trace!(target: "exec","Method Type: {}", class_name);
                                                type_final = class_name;
                                            }
                                        }

                                        execute_internel_function(
                                            class_name_final,
                                            name_final,
                                            type_final,
                                            &mut operand_stack,
                                        );
                                    }
                                    _ => panic!("Expected Method Ref"),
                                }
                            }
                            96 =>{
                                let value1 = operand_stack.pop().unwrap();
                                let value2 = operand_stack.pop().unwrap();
                                if let Varubals::Int(a) = value1{
                                    if let Varubals::Int(b) = value2{
                                        operand_stack.push(Varubals::Int(a+b))
                                    }
                                }
                            }
                            60 =>{
                                local_varuble_array[1] = operand_stack.pop().unwrap();
                            }
                            61 =>{
                                local_varuble_array[2] = operand_stack.pop().unwrap();
                            }
                            62 =>{
                                local_varuble_array[3] = operand_stack.pop().unwrap();
                            }
                            27 =>{
                                operand_stack.push(local_varuble_array[1].clone())
                            }
                            29 =>{
                                operand_stack.push(local_varuble_array[3].clone())
                            }
                            28 =>{
                                operand_stack.push(local_varuble_array[2].clone())
                            }
                            42 => {
                                operand_stack.push(local_varuble_array[0].clone());
                            }
                            5 => {
                                operand_stack.push(Varubals::Int(2))
                            }
                            6 => {
                                operand_stack.push(Varubals::Int(3))
                            }
                            3 => {
                                operand_stack.push(Varubals::Int(0))
                            }
                            4 => {
                                operand_stack.push(Varubals::Int(1))
                            }
                            16 => {
                                let byte:u8 = bytecode.read_be().unwrap();
                                operand_stack.push(Varubals::Int(byte.into()));
                            }
                            54 => {
                                let index:u8 = bytecode.read_be().unwrap();
                                local_varuble_array[index as usize] = operand_stack.pop().unwrap();
                            }
                            _ => {
                                panic!("Unknown opcode: {}", opcode)
                            }
                        }
                    }
                
                
                
                
                }
                attributes::AttributeInfo::SourceFile(_) => todo!(),
            }
        }
    }
}
