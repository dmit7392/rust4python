use std::{fs::read_to_string, io::{stdin, stdout, Write, }};
use std::fs::write;
use anyhow::Result;

use std::str::FromStr;

use crate::car::Car;

pub struct CarToolApp {
    pub cars: Vec<Car>,
}

impl CarToolApp {
    pub fn new() -> CarToolApp {
        CarToolApp {cars : vec![
            Car::new(0, "Honda", "Civic", 2008, "?", 1000.0),
            Car::new(1, "Toyota", "Tundra", 2008, "?", 2000.0),
            Car::new(2, "Ford", "Ka", 2008, "?", 100.0),
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
            self.cars.len(),
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

        let total_value: f64 = self.cars.iter().map(|car|car.price).sum();
        println!("Cars # {}, Total value: {total_value}", self.cars.len());
    }

    pub fn remove_car(&mut self) -> Result<usize> {
        let id:usize = Self::console_read("id?")?;
        let index = self.cars.iter().position(|car| car.id == id);

        match index {
            Some(i) => {
                self.cars.remove(i);
                println!("Car with id {} removed successfully", id);
            }
            None => println!("Car with id {} not found", id),
        }

        Ok(self.cars.len())
    }

    pub fn load_cars(&mut self) -> Result<()> {
        let file_name: String = Self::console_read("file?")?;
        let jcars = read_to_string(file_name.as_str())?;
        let cars: Vec<Car> = serde_json::from_str(&jcars)?;
        self.cars = cars;
        Ok(())
    }

    pub fn save_cars(&self) -> Result<()> {
        let file_name: String = Self::console_read("file?")?;
        let jcars = serde_json::to_string(&self.cars)?;
        write(file_name.as_str(), jcars)?;
        Ok(())
    }

    
}
