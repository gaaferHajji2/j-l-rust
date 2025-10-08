fn add(t1: i32, t2: i32) -> i32 {
    return t1 + t2;
}

fn mul(t1: f64, t2: f64) -> f64 {
    t1 * t2 // here if we don't set ; then it will be used as: return t1 * t2;
}

fn main() {
    println!("The add is: {}", add(1, 2));

    println!("The mul is: {}", mul(2.1, 3.2));

    println!("Hello, world!");
}