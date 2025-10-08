fn main() {

    let number: i32 = 3;

    match number {
        1 => println!("The number is 1"),
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        _ => println!("Something else"),
    }

    let number: i32 = 7;

    match number {
        1..=5 => println!("Number between 1 and 5"),
        6..=10 => println!("Number between 6 and 10"),
        _ => println!("Something else"), // if we don't set it, it will give error
    }

    println!("Hello, world!");
}
