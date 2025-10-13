
fn add(t1: i32, t2: i32) -> i32 {
    return t1 + t2;
}

fn mul(t1: f64, t2: f64) -> f64 {
    t1 * t2 // here if we don't set ; then it will be used as: return t1 * t2;
}

// test same lifetime for variables
fn longest<'a> (t1: &'a str, t2: &'a str) -> &'a str {
    if t1.len() > t2.len() {
        t1
    } else {
        t2
    }
}

fn main() {
    println!("The add is: {}", add(1, 2));
    println!("The mul is: {}", mul(2.1, 3.2));
    println!("Hello, world!");

    let t1: String = String::from("Hello Jafar Loka String Function");

    println!("The msg is: {}", t1);

    let t2: String = String::from("Jafar-Loka-01");
    let t3: String = String::from("Jafar-Loka-02-ITE Developer");

    let result: &str = longest(&t2, &t3);
    println!("The longest string is: {}", result)
}