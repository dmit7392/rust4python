use std::io::{stdin, stdout, Write, };
use anyhow::Result;

use std::str::FromStr;

use crate::car::Car;

pub struct CarToolApp {
    pub cars: Vec<Car>,
}

impl CarToolApp {
    pub fn new() -> CarToolApp {
        CarToolApp {cars : vec![
            Car::new("Honda", "Civic", 2008, "?", 1000.0),
            Car::new("Toyota", "Tundra", 2008, "?", 2000.0),
            Car::new("Ford", "Ka", 2008, "?", 100.0),
        ]}
    }

    pub fn console_read<T: FromStr>(prompt: &str) -> Result<T> where <T as FromStr>::Err: std::error::Error + Send + Sync + 'static {
        print!("{prompt} ");
        stdout().flush()?;
        
        let mut str = String::new();
        stdin().read_line(&mut str)?;
    
        Ok(str.trim().parse::<T>()?)
    }
    
    pub fn add_car(&mut self) -> Result<usize> {
        let make: String = Self::console_read("make?")?;
        let model: String = Self::console_read("model?")?;
        let color: String = Self::console_read("color?")?;

        let car = Car::new(
            &make,
            &model,
            Self::console_read("year?")?,
            &color,
            Self::console_read("price?")?
        );

        self.cars.push(car);
    
        Ok(self.cars.len())
    }

    pub fn show_cars(&self) {
        for car in &self.cars {
            println!("{}", car);
        }

        let mut totalValue: f64 = std::f64::NAN;

        totalValue = self.cars.iter().map(|car|car.price).sum();

        println!("Cars # {}, Total value: {totalValue}", self.cars.len());
    }
    
    
}
