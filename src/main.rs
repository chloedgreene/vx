use binread::{BinRead, io::Cursor, BinReaderExt};

const HELLOWORLD: &[u8; 417] = include_bytes!("../code/Hello.class");
fn main() {
    
    let mut reader = Cursor::new(HELLOWORLD);
    if reader.read_be::<u32>().unwrap() != 0xCAFEBABE{ // checks magic id of class files
        panic!("Magic is not valid")
    }

    
    

}  
