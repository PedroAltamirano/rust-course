fn main() {
    // panic!("panic message")

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // panics
    // println!("{:?}", fruits[5]);

    let first = fruits.get(5);
    println!("{:?}", first);

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {}
    }
    // equivalents
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }

    // unwrap
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy");

    // try - catch
    #[derive(Debug)]
    struct DivisionByZeroError;

    fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
        if divisor == 0.0 { Err(DivisionByZeroError) } else { Ok(dividend / divisor) }
    }

    fn main() {
        println!("{:?}", safe_division(9.0, 3.0));
        println!("{:?}", safe_division(4.0, 0.0));
        println!("{:?}", safe_division(0.0, 2.0));
    }
}