use crate::Varubals;

pub fn execute_internel_function(
    class: &str,
    name: &str,
    tpye: &str,
    local_stack: &mut Vec<Varubals>,
) {
    // spelling mistake intentional, ask chloe

    match class {
        "java/lang/Object" => match name {
            "<init>" => {
                println!("Called constructor on object")
            }

            _ => panic!("Unsupported method name, does not exsist :{}", name),
        },

        "java/io/PrintStream" => match name {
            "println" => {
                if let Varubals::String(d) = local_stack.pop().unwrap() {
                    println!("{}", d)
                }
            }

            _ => panic!("Unsupported method name, does not exsist :{}", name),
        },

        _ => panic!("Unsupported class, does not exsist :{}", class),
    }
}
