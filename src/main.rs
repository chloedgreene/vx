use std::{env, process::exit};

use binread::{io::Cursor, BinReaderExt};

use classruntime::class_runtime;
use constpool::ConstantPoolTags;
use log::{info, trace};
use method::MethodInfo;

use crate::{constpool::get_constant_pool, attributes::{Attribute, get_atributes}, class::load_classes};

mod attributes;
mod constpool;
mod method;
mod stubs;
mod classruntime;
mod class;



fn main(){

    //get arguments for the calss files
    let args: Vec<String> = env::args().collect();
    env_logger::builder()
    .init();

    let path_list = match args.len() { // all paths to classes

        1 => {println!("Please Specify A Class File"); exit(1);}
        2 => {&args[1..args.len()]}

        _ => {panic!("Idk how to respond to : {}",args.len())}
    };

    let classes:Vec<class::class> = load_classes(path_list);

    // if cfg!(debug_assertions) { //this handy line of code only runes in debug builds of the app
    //     println!("{:?}",classes);
    // }

    let runtime_classes:Vec<class_runtime> = classes.iter().map(|f| {
        class_runtime::new(f.clone())
    }).collect();

    print!("{:?}",runtime_classes);
    

}
