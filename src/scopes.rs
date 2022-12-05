fn process(input: String) {}

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Ownership of the string in `s` moved into `process`
    process(s); // Error! ownership already moved.

    // use inmutability
    let s = String::from("Hello, world!");
    process(s.clone()); // Passing another value, cloned from `s`.
    process(s); // s was never moved and so it can still be used.

    // use pointers
    let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting); // We can still use `greeting`
}

fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

// mutable ponters
fn change(text: &mut String) {
    text.push_str(", world");
}

fn main() {
    // block scope
    // `mascot` is not valid and cannot be used here, because it's not yet declared.
    {
        let mascot = String::from("ferris"); // `mascot` is valid from this point forward.
        // do stuff with `mascot`.
    }
    // this scope is now over, so `mascot` is no longer valid and cannot be used.
    println!("{}", mascot);

    {
        let mascot = String::from("ferris");
        // transfer ownership of mascot to the variable ferris.
        let ferris = mascot;
        // ferris dropped here. The string data memory will be freed here.
        // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
        println!("{}", mascot);
    }

    // use pointers
    let greeting = String::from("Hello");
    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again

    // mutable pointers
    let mut greeting = String::from("hello");
    change(&mut greeting);

    // declare several pointers
    let ref1 = &value;
    // declare just one pointer
    let ref2 = &mut value;
}