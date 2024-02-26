use std::{env, process::exit};



use classruntime::class_runtime;




use crate::{class::load_classes};

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

    let constructedd_classes:Vec<class_runtime> = classes.iter().map(|f| {
        let mut current_class = class_runtime::new(f.clone());

        current_class.run_method("<init>".to_owned());

        current_class
    }).collect();
    


    

}
