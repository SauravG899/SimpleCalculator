use std::io;
use std::io::{Write};

fn read(input: &mut String) {
    io::stdout().flush().expect("failed to flush");
    io::stdin().read_line(input).expect("failed to read");
}

pub fn input() {

    loop {
        let mut num_1 = String::new();
        let mut num_2 = String::new();
        let mut operator = String::new();
        

        print!("Operation [+-*/^m]: ");
        read(&mut operator);

        print!("Enter first number (if exponent, base/if modulus, a ): ");
        read(&mut num_1);

        print!("Enter second number (if exponent power/if modulus, b): ");
        read(&mut num_2);

        let num_1: f32 = num_1.trim().parse().unwrap();
        let num_2: f32 = num_2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        let operators = String::from("+ - * / ^ r ");

        if !operators.contains(operator) {
            println!("unknown operator");
            continue;
        }

        let result = match operator {
            '+' => num_1 + num_2,
            '-' => num_1 - num_2,
            '*' => num_1 * num_2,
            '/' => num_1 / num_2,
            '^' => num_1.powf(num_2),
            'r' => num_1 % num_2,

        
            _ => panic!("error in operator")
        };

        println!("the result of {} {} {} = {}", num_1, operator, num_2, result);
    }
}

