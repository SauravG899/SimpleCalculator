mod stats_calculator;
mod quadratic;
mod calculator;

use std::io;

fn main(){
    
    println!("Which function would you like to use? (Stats (a), General (b), Quadratic (c) )");
    
    let mut _typeofcalc_buffer = String::new();
    
    io::stdin().read_line(& mut _typeofcalc_buffer).unwrap();

    let choiceoffunc = _typeofcalc_buffer.trim_end();

    match choiceoffunc {
        "a" => stats_calculator::run(),
        "b" => calculator::input(),
        "c" => todo!("Coming Soon!"),
        _ => panic!(),
    };
}
