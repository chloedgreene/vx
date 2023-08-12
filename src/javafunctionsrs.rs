use log::{info, trace};

use crate::Varubals;

pub fn execute_internel_function(
    class: &str,
    name: &str,
    _tpye: &str,
    local_stack: &mut Vec<Varubals>,
) {
    // spelling mistake intentional, ask chloe


    trace!(target: "exec","Running {}.{}:{}",class,name,_tpye);

    match class {
        "java/lang/Object" => match name {
            "<init>" => {
                info!(target: "class","Called constructor on object")
            }

            _ => panic!("Unsupported method name, does not exsist :{}", name),
        },

        "java/io/PrintStream" => match name {
            "println" => {
                // if let Varubals::String(d) = local_stack.pop().unwrap() {
                //     println!("{}", d)
                // }
                // if let Varubals::Int(d) = local_stack.pop().unwrap() {
                //     println!("{}", d)
                // }

                let value = local_stack.pop().unwrap();
            
                match &value {

                    Varubals::String(g) =>{
                        println!("{}",g)
                    }

                    Varubals::Int(g) =>{
                        println!("{}",g)
                    }

                    _ => {panic!("Cant print {:?}",value)}
                    
                }

            }


            _ => panic!("Unsupported method name, does not exsist :{}", name),
        },

        _ => panic!("Unsupported class, does not exsist :{}", class),
    }
}
