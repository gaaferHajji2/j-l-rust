use std::io;

fn main() {
    println!("Please enter your age: ");

    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: &str = input.trim();

    let input: u32 = input.parse().expect("Please enter a number!");

    println!("Your age is: {input}");
}
