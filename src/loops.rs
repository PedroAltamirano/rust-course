fn main() {
  loop {
    println!("Hello loop");
    break;
  }

  let mut counter = 0;
  while counter < 5 {
    println!("We loop a while...");
    counter = counter + 1;
  }

  let big_birds = ["ostrich", "peacock", "stork"];
  for bird in big_birds.iter() {
    println!("The {} is a big bird.", bird);
  }

  for number in 0..5 {
    println!("{}", number * 2);
  }
}