//define enum
enum Shape {
    J01(i32),
    J02(f64, f64)
}

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

    // match tuples
    let point: (i32, i32) = (0, 7);

    match point {
        (0, y) => println!("Point on y, {}", y),
        (x, 0) => println!("Point on x, {}", x),
        (x, y)=> println!("Point on x, y, {}, {}", x, y)
    };

    // matching enum
    let t1 = Shape::J02(1.1, 1.2);

    let _ = Shape::J01(1);

    match t1 {
        Shape::J01(t2) => println!("The shape is J01 ==> {}", t2),
        Shape::J02(t3, t4) => println!("The shape is J02 ==> {}, {}", t3, t4),
    };

    println!("Hello, world!");
}
