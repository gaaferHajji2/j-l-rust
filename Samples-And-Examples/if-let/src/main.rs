fn main() {
    let t1 = Some(1);
    if let Some(t2) = t1 { // here we use = not ==
        println!("t2 is {}", t2)
    } else {
        println!("t2 is...")
    }

    println!("Hello, world!");
}
