use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Car {
    pub id: usize,
    make: String,
    model: String,
    year: u32,
    color: String,
    pub price: f64,
}

impl Car {
    pub fn new (id: usize, make: &str, model: &str, year: u32, color: &str, price: f64) -> Car {
        Car {
            id,
            make: make.to_string(),
            model: model.to_string(),
            year,
            color: color.to_string(),
            price,
        }
    }

    // pub fn print(&self) {
    //     println!("CAR: {} {}", self.make, self.model);
    // }
}

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CAR: {} {} - ${:.2}", self.make, self.model, self.price)
    }
}
