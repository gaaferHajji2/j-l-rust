
macro_rules! greet {
    ($name:expr) => { // Here also we can pass no arguments
        println!("Hello {}!", $name)
    }
}

macro_rules! greet2 {
    () => { // Here also we can pass no arguments
        println!("Hello Jafar Loka!")
    }
}

fn main() {
    println!("Hello, world!");

    greet!("Jafar Loka");

    greet!("Black Dragons");

    greet2!();
}
