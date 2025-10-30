use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("Hello, world!");

    println!("{:?}", args);

    // accessing args
    let pro_path: &str = &args[0];
    println!("The path of our program: {}", pro_path)
}
