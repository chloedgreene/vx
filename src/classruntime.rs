use crate::class::class;

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

pub struct frame{

}

pub struct class_runtime{
    class: class,
    pc: usize, // proboly overkill but w h y n o t
    //lva: Vec<Varubals> //= vec![Varubals::Null; 256]
    frames: Vec<frame>
}
impl class_runtime{
    pub fn new(x:class) -> Self{
        return class_runtime{
            class: x,
            pc: 0,
            frames: vec![],
        };
    }
}