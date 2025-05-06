use std::io::{stdin, stdout, Write};

use anyhow::Result;

//Ex 2

struct Car {
    make: String,
    model: String,
    year: u32,
    color: String,
    price: f64,
}

impl Car {
    fn new (make: &str, model: &str, year: u32, color: &str, price: f64) -> Car {
        Car {
            make: make.to_string(),
            model: model.to_string(),
            year,
            color: color.to_string(),
            price,
        }
    }

    fn print(&self) {
        println!("CAR: {} {}", self.make, self.model);
    }
}

fn read_u32(prompt: &str) -> Result<u32> {
    Ok(loop {
        let mut input = String::new();
        print!("{prompt} ");

        stdout().flush()?;

        stdin().read_line(&mut input)?;
        
        match input.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
    })
}

fn read_string(prompt: &str) -> Result<String> {
    let mut str = String::new();
    print!("{prompt} ");
    stdout().flush()?;
    
    stdin().read_line(&mut str)?;
    Ok(str)
}

//Ex. 1
fn num_input(prompt: &str) -> Result<f64> {
    Ok(loop {
        let mut input = String::new();
        print!("{prompt} ");

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

    // Ex.2

    let make = read_string("make?").expect("make...");
    let model = read_string("model?").expect("model...");
    let year = read_u32("year?").expect("year...");
    let color = read_string("color?").expect("color...");
    let price = num_input("price?").expect("price...");

    let car = Car::new(&make, &model, year, &color, price);
    car.print();
}
