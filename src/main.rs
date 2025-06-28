use std::io;


fn main() {
    println!("Hello, world!");

    println!("Guess a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read value");

    println!("You guessed: {}", input);
}
