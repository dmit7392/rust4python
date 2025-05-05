//Ex. 1

use std::io::{stdin, stdout, Write};

use anyhow::Result;

fn num_input(prompt: &str) -> Result<f64> {
    Ok(loop {
        let mut input = String::new();
        print!("{}", prompt);

        stdout().flush()?;

        stdin().read_line(&mut input)?;
        
        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    })
}

fn main() {
    let num1 = num_input("Enter the first number > ").map_err(|_| "Failed to read the first number".to_string()).unwrap();
    let num2 = num_input("Enter the second number > ").map_err(|_| "Failed to read the second number".to_string()).unwrap();
    
    let result = num1 + num2;
    println!("The sum of {} and {} is {}", num1, num2, result);
}
