// Create a struct called "Car" with fields like "brand," "model," and "year."
// Implement an enum called "FuelType" with variants like "Gasoline," "Diesel," and "Electric."
// Write a function that takes a Car object and a FuelType enum variant and prints out the car's details along with the fuel type.

pub mod showroom {

    pub struct Car {
        pub brand: String,
        pub model: String,
        pub year: u32,
    }
    #[derive(Debug)]
    pub enum FuelType {
        Gasoline,
        Diesel,
        Electric,
    }
    pub fn matching_car(car: &Car, ft: FuelType) {
        println!(
            "Details\nBrand: {}\nModel: {}\nYear: {}\nFuel: {}",
            car.brand,
            car.model,
            car.year,
            match ft {
                FuelType::Gasoline => "Gasoline",
                FuelType::Diesel => "Diesel",
                FuelType::Electric => "Electric",
            }
        );
    }
}
use showroom::*;
fn main() {
    let mycar = Car {
        brand: String::from("Tesla"),
        model: String::from("Y"),
        year: 2023,
    };
    matching_car(&mycar, FuelType::Diesel);
}
