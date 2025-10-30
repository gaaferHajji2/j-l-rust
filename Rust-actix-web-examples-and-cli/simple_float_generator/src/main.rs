use rand::prelude::*;

fn generate_float(generator: &mut ThreadRng) -> f64 {
    let t1: f64 = generator.random();
    return t1 * 10.0
}

fn main() {

    let mut t1: ThreadRng = rand::rng();

    println!("float value is: {}", generate_float(&mut t1));

    println!("Hello, world!");
}
