use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: (Age, u32),
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles);
    }

    (Age::New, 0)
}

fn car_factory(color: String, transmission: Transmission, convertible: bool, miles: u32) -> Car {
    if miles > 0 {
        if !convertible {
            println!(
                "Prepare a used car: {:?}, {}, Hard top, {} miles\n",
                transmission,
                color,
                miles
            );
        }
    }

    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: car_quality(miles),
    }
}

fn main() {
    let mut orders: HashMap<i32, Car> = HashMap::new();
    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut order = 1;
    let mut engine: Transmission = Transmission::Manual;
    let mut car: Car = car_factory(String::from(colors[0]), engine, true, 0);
    orders.insert(order, car);

    order = 2;
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    orders.insert(order, car);

    order = 3;
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    orders.insert(order, car);

    for car_order in 1..4 {
        println!("Car order {}: {:?}", car_order, orders.get(&car_order));
    }
}