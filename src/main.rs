use std::{task::Context, io::Read};

use binread::{BinRead, io::Cursor, BinReaderExt};
use constpool::ConstantPoolTags;

use crate::{constpool::{get_constant_pool}, method::method_info, attributes::{attribute, get_atributes}};

mod constpool;
mod method;
mod attributes;

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

enum MethodAccessFlag {
    PUBLIC      	= 0x0001,
    PRIVATE 	    = 0x0002,
    PROTECTED 	    = 0x0004,
    STATIC 	        = 0x0008,
    FINAL        	= 0x0010,
    SYNCHRONIZED 	= 0x0020,
    BRIDGE 	        = 0x0040,
    VARARGS 	    = 0x0080,
    NATIVE 	        = 0x0100,
    ABSTRACT 	    = 0x0400,
    STRICT 	        = 0x0800,
    SYNTHETIC 	    = 0x1000
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
    }


}  
