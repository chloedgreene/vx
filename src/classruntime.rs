use crate::{class::class, constpool::ConstantPoolTags};
use binread::{io::Cursor, BinReaderExt};
use clap::builder::Str;

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


#[derive(Debug, Clone)]

pub struct frame{
    name: String,
    local_varuble_array: Vec<Varubals>,
    operand_stack: Vec<Varubals>,
    bytecode : Vec<u8>

}

#[derive(Debug, Clone)]

pub struct class_runtime{
    class: class,
    pc: usize, // proboly overkill but w h y n o t
    //lva: Vec<Varubals> //= vec![Varubals::Null; 256]
    frames: Vec<frame>
}
impl class_runtime{
    pub fn new(x:class) -> Self{
    
        //we now generated frames
        
        return class_runtime{
            class: x.clone(),
            pc: 0,
            frames: gen_frames(x),
        };
    }
    pub fn run_method(&mut self,name: String){
        for f in &self.frames{
            if f.name == name{
                let mut current_frame = f.clone();
                self.exec(&mut current_frame);
            }
        }
    }

    pub fn exec(&mut self,f: &mut frame){
        let length = f.bytecode.len();
        let mut bytecode = Cursor::new(f.bytecode.clone());




        for i in 0..length {
            let opcode: u8 = bytecode.read_be().unwrap();
            match opcode {
                18 => {
                    let index: u8 = bytecode.read_be().unwrap();
                    let value = self.class.constant_pool[index as usize - 1];

                    match value {
                        ConstantPoolTags::String(utf8_index) => {
                            if let ConstantPoolTags::Utf8(_, data) =
                                self.class.constant_pool[utf8_index as usize - 1]
                            {
                                f.operand_stack.push(Varubals::String(data.to_string()))
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
                    let fieldref = &self.class.constant_pool[index as usize - 1];
                    if let ConstantPoolTags::Fieldref(a, b) = fieldref {
                        let class_index = &self.class.constant_pool[*a as usize - 1];
                        if let ConstantPoolTags::Class(u) = class_index {
                            let utf8 = self.class.constant_pool[*u as usize - 1];
                            if let ConstantPoolTags::Utf8(_, class_name) = utf8 {
                                class_name_final = &class_name;
                            }
                        }

                        let name_and_type_index = self.class.constant_pool[b as usize - 1];
                        if let ConstantPoolTags::NameAndType(u, j) = name_and_type_index
                        {
                            let utf81 = self.class.constant_pool[u as usize - 1];
                            let utf82 = self.class.constant_pool[j as usize - 1];

                            if let ConstantPoolTags::Utf8(_, class_name) = utf81 {
                                name_final = &class_name.clone();
                            }
                            if let ConstantPoolTags::Utf8(_, class_name) = utf82 {
                                type_final = &class_name;
                            }
                        }
                    }

                    f.operand_stack.push(Varubals::Class(
                        class_name_final.to_string(),
                        name_final.to_string(),
                        type_final.to_string(),
                    ));
                }
                177 => {
                    break;
                }
                183 | 182 => {
                    let index: u16 = bytecode.read_be().unwrap();
                    let methodred = &self.class.constant_pool[index as usize - 1];
                    match methodred {
                        ConstantPoolTags::Methodref(a, b) => {
                            let mut class_name_final = "";
                            let class_index = &self.class.constant_pool[*a as usize - 1];
                            if let ConstantPoolTags::Class(u) = class_index {
                                let utf8 = &self.class.constant_pool[*u as usize - 1];
                                if let ConstantPoolTags::Utf8(_, class_name) = utf8 {
                                    class_name_final = &class_name;
                                }
                            }
                            let mut name_final = "";
                            let mut type_final = "";

                            let name_and_type_index = &self.class.constant_pool[*b as usize - 1];
                            if let ConstantPoolTags::NameAndType(u, j) =
                                name_and_type_index
                            {
                                let utf81 = &self.class.constant_pool[*u as usize - 1];
                                let utf82 = &self.class.constant_pool[*j as usize - 1];

                                if let ConstantPoolTags::Utf8(_, class_name) = utf81 {
                                    name_final = &class_name;
                                }
                                if let ConstantPoolTags::Utf8(_, class_name) = utf82 {
                                    type_final = &class_name;
                                }
                            }

                            // execute_internel_function(
                            //     class_name_final,
                            //     name_final,
                            //     type_final,
                            //     &mut operand_stack,
                            // );

                            panic!("havent implimented internal function, chloe add that rn")
                        }
                        _ => panic!("Expected Method Ref"),
                    }
                }
                96 =>{
                    let value1 = f.operand_stack.pop().unwrap();
                    let value2 = f.operand_stack.pop().unwrap();
                    if let Varubals::Int(a) = value1{
                        if let Varubals::Int(b) = value2{
                            f.operand_stack.push(Varubals::Int(a+b))
                        }
                    }
                }
                60 =>{
                    f.local_varuble_array[1] = f.operand_stack.pop().unwrap();
                }
                61 =>{
                    f.local_varuble_array[2] = f.operand_stack.pop().unwrap();
                }
                62 =>{
                    f.local_varuble_array[3] = f.operand_stack.pop().unwrap();
                }
                27 =>{
                    f.operand_stack.push(f.local_varuble_array[1].clone())
                }
                29 =>{
                    f.operand_stack.push(f.local_varuble_array[3].clone())
                }
                28 =>{
                    f.operand_stack.push(f.local_varuble_array[2].clone())
                }
                42 => {
                    f.operand_stack.push(f.local_varuble_array[0].clone());
                }
                5 => {
                    f.operand_stack.push(Varubals::Int(2))
                }
                6 => {
                    f.operand_stack.push(Varubals::Int(3))
                }
                3 => {
                    f.operand_stack.push(Varubals::Int(0))
                }
                4 => {
                    f.operand_stack.push(Varubals::Int(1))
                }
                16 => {
                    let byte:u8 = bytecode.read_be().unwrap();
                    f.operand_stack.push(Varubals::Int(byte.into()));
                }
                54 => {
                    let index:u8 = bytecode.read_be().unwrap();
                    f.local_varuble_array[index as usize] = f.operand_stack.pop().unwrap();
                }
                _ => {
                    panic!("Unknown opcode: {}", opcode)
                }
            }
        }

    }


}

fn gen_frames(class: class) -> Vec<frame>{


    let t:Vec<frame> = class.methods.iter().map(|m|{
        let method_name = &class.constant_pool[m.name_index as usize -1];
        let mut name = "";
        if let ConstantPoolTags::Utf8(_,data) = method_name{ // we need to use a if let to reassign the value in memory to a new position in memory so we can free space inbewtween 2 memory blocks and use the Box<> datatype to include the heap in this gap, this makes the linux kernel happy so i gotta do it, i why did i decide to start wrining my own verison of java in rust lmfao
            name = data;
        }else {
            panic!("The methods name is linked against a invalid CP-TAG")
        }

        let mut bytes = vec![];
        //now we load in the atributes
        for attr in &m.attributes{

            match &attr.info {
                crate::attributes::AttributeInfo::LineNumberTable(_, _) => {}, // we not adding a debugger so we can safely ignore
                crate::attributes::AttributeInfo::Code(_, _, _, code, _, _, _source_lines) => {
                    //LOOK WERE FINALLY HERE
                    //ITS THE ACTUAL PART OF THE CODE THAT LOADES C O  D E INTO IT 
                    // YIPEEEEEEEEEEEEEEEEe
                    //also it dosent run it, we just make stuff
                    bytes = code.to_vec();
                                   
                },
                crate::attributes::AttributeInfo::SourceFile(_) => {}, //  still no debugger
            }
        }

        return frame{
            name: name.to_owned(),
            local_varuble_array: {
                let mut temp = vec![Varubals::Null; 256];
                temp[0] = Varubals::Reference(ReferenceType::Constpool(class.this_class));
                temp
            },
            operand_stack: vec![],
            bytecode: bytes.to_vec(),
        };

    }).collect();

    return t;
}