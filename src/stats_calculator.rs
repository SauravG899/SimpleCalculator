use std::collections::HashMap;
use std::io;
use std::str::FromStr;


pub enum OperationType {
    Mean,
    Median,
    Mode,
 
}

impl FromStr for OperationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "mean" => Ok(Self::Mean),
            "median" => Ok(Self::Median),
            "mode" => Ok(Self::Mode),
            // "quadratic" => Ok(Self::Quadratic),
            _ => Err(String::from("Invalid input!"))
        }
    }
}

pub fn do_operation(list: &Vec<f64>, op_type: OperationType) -> f64 {
    match op_type {
        OperationType::Mean => {
            let mut sum = 0.0;
            for val in list {
                sum += val;
            }
            sum / (list.len() as f64)
        },
        OperationType::Median => {
            let mut sorted_list = list.clone();
            sorted_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if sorted_list.len() % 2 == 0 {
                let num1 = sorted_list[sorted_list.len() / 2 - 1];
                let num2 = sorted_list[sorted_list.len() / 2];
                (num1 + num2) / 2.0
            } else {
                sorted_list[sorted_list.len() / 2]
            }
        },
        OperationType::Mode => {
            let mut hash_map: HashMap<String, usize> = HashMap::new();
            for val in list {
                let entry = hash_map.entry(val.to_string()).or_insert(0);
                *entry += 1;
            }

            let mut max: (f64, usize) = (0.0, 0);
            for pair in hash_map {
                if pair.1 > max.1 {
                    max.1 = pair.1;
                    max.0 = pair.0.parse().unwrap();
                }
            }
            max.0
        }
    }
}

pub fn run() {
    println!("Enter a dataset: (ex. 4,2,53,-2,4.3,632.4245,-3.35)");
    let mut dataset_buffer = String::new();
    io::stdin().read_line(&mut dataset_buffer).unwrap();

    let dataset: Vec<f64> = dataset_buffer.split(',').map(|elem| elem.trim().parse::<f64>().unwrap()).collect();

    println!("Enter the operation type: (ex. 'Mean', 'Median', 'Mode', 'All')");
    let mut choice_buffer = String::new();
    io::stdin().read_line(&mut choice_buffer).unwrap();
    if choice_buffer.trim() == String::from("All") {
        println!(
            "The answer is Mean: {0}, Median: {1}, Mode: {2}", 
            do_operation(&dataset, OperationType::Mean),
            do_operation(&dataset, OperationType::Median),
            do_operation(&dataset, OperationType::Mode)
        );
    } else {
        println!("The answer is {}", do_operation(&dataset, choice_buffer.trim().parse().unwrap()));
    }
}
