#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,
    }
}

fn main() {
    let car: Car = car_factory(String::from("Black"), Transmission::Manual, true);
    println!(
        "Car => {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color,
        car.transmission,
        car.convertible,
        car.mileage
    );
}