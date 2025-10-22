fn main() {
    let t1 = Some(1);
    if let Some(t2) = t1 { // here we use = not ==
        println!("t2 is {}", t2)
    } else {
        println!("t2 is...")
    }

    let t3: Result<i32, &str> = Ok(1);

    if let Ok(t4) = t3 {
        println!("The value of t4 is: {}", t4);
    } else {
        println!("The value is ....")
    }

    println!("Hello, world!");
}
