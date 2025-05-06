use std::io::{stdin, stdout, Write, };
use anyhow::Result;

use crate::car::Car;

pub struct CarToolApp {
    pub cars: Vec<Car>,
}

impl CarToolApp {
    pub fn new() -> CarToolApp {
        CarToolApp {cars : vec![]}
    }

    pub fn read_string(prompt: &str) -> Result<String> {
        print!("{prompt} ");
        stdout().flush()?;
        
        let mut str = String::new();
        stdin().read_line(&mut str)?;
    
        Ok(str.trim().to_string())
    }
    
    pub fn read_u32(prompt: &str) -> Result<u32> {
        Ok(loop {
            print!("{prompt} ");
            stdout().flush()?;
            
            let mut input = String::new();
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
    
    pub fn num_input(prompt: &str) -> Result<f64> {
        Ok(loop {
            print!("{prompt} ");
            stdout().flush()?;
            
            let mut input = String::new();
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

    pub fn add_car(&mut self) -> Result<usize> {
        let make = Self::read_string("make?")?;
        let model = Self::read_string("model?")?;
        let year = Self::read_u32("year?")?;
        let color = Self::read_string("color?")?;
        let price = Self::num_input("price?")?;
    
        let car = Car::new(&make, &model, year, &color, price);
        self.cars.push(car);
    
        Ok(self.cars.len())
    }

    pub fn show_cars(&self) {
        for car in &self.cars {
            car.print();
        }
    }
    
    
}
