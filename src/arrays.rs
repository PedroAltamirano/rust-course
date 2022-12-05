fn main() {
    let tuple = ("word", 4, true);
    println!("{:?}", tuple.0);

    let array = [0; 5];
    println!("{:?}", array);

    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);
    println!("Last fruit: {:?}", fruit.pop());
}
