fn main() {

    let number: i32 = 7;

    if number < 10 {
        println!("Condition is TRUE");
    }

    if number > 10 {
        println!("ERROR!!!")
    } else {
        println!("TRUE!!!");
    }

    if number > 10 {
        println!("ERROR!!!");
    } else if number > 5 {
        println!("TRUE!!!");
    } else {
        println!("ERROR!!!");
    }

    println!("Hello, world!");

    if number > 1 && number < 8 {
        println!("Number Between 1 and 8");
    }

    let cond: bool = true;

    let number: i32 = if cond { 10 } else { 5 };

    println!("New Number is: {number}")
}
