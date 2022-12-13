fn add(a: i32, b: i32) -> i32 {
  a + b
}

// sucess test
#[test]
fn add_works() {
  assert_eq!(add(1, 2), 3);
  assert_eq!(add(10, 12), 22);
  assert_eq!(add(5, -2), 3);
}

// error test
#[test]
#[should_panic]
fn add_fails() {
  assert_eq!(add(2, 2), 7);
}

// igoner test
#[test]
#[ignore = "not yet reviewed by the Q.A. team"]
fn add_negatives() {
  assert_eq!(add(-2, -2), -4)
}

fn main() {
  let res = add(5, 4);
  println!("5+4={}", res)
}
