fn main() {
    let t1 = 5;
    match t1 {
        t2 if t2 % 2 == 0 => println!("The number is even"),
        _ => println!("The number is odd")
    }
    println!("Hello, world!");
}
