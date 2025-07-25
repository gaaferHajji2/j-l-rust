fn main() {

    let y = 10;

    // y = 20; // This Will Raise Compile Error

    let mut x = 20;

    println!("The Value Of X, Before Changing Is: {}", x);

    x = 25; // If We Don't Use The Previous Value, Warning Will Appear

    println!("Hello, world!, y is: {}, x is: {}, x+y is: {}", y, x, y+x);


    /*
     * Testing The Integer Type
     */
    let a1 = 98_999;
    let a2 = 0xfff;
    let a3 = 0o337;
    let a4 = 0b1111_0000;
    let a5 = b'a';

    println!("a1 is: {}, a2 is: {}, a3 is: {}, a4 is: {}, a5 is: {}", a1, a2, a3, a4, a5);

    /*
     * Testing Floating Types
     */
    let a6 = 2.5;
    let a7: f32 = 4.9;

    println!("a6 is: {}, a7 is: {}", a6, a7);

    /*
     * Testing Boolean Values
    */
    let t = true;
    let f = false;

    println!("The True Value Is: {}", t);
    println!("The False Value Is: {}", f);

    // Test chars
    let a = 'a';
    let b: char = 'b';
    let c: char = '😊';

    println!("The value of a is: {}, The value of b is: {}, the value of c is: {}", a, b, c);

    // Testing tuples
    let tup: (i32, f64, u8) = (500, 6.5, 1);

    // We can destructing tuple:
    let (x, y, z) = tup;

    println!("The value of x is: {}, the value of y is: {}, the value of z is: {}", x, y, z);

    // Access the tuple element directly
    let x1 = tup.0;
    let y1: f64 = tup.1;
    let z1  = tup.2;

    println!("The value of x1 is: {}, the value of y1 is: {}, the value of z1 is: {}", x1, y1, z1);

    // Testing Arrays
    let arr = [1, 2, 3, 4, 5];

    // println!("The Array is: {}", arr); // This will give compiler error

    let temp = arr[3];

    println!("The 1st value is: {}, the 2nd value is: {}, the 3rd value is: {}, the 4th value: {temp}", 
        arr[0], arr[1], arr[2]);
    
    // for better memory safety, when dealing with arrays and indexes
    let index = 3;

    match arr.get(index) {
        Some(element) => println!("The element is: {element}"),
        None => println!("Please check the index"),
    }

    // Testing Constants
    const MAX_USERS: u32 = 1000;

    println!("The max users is: {MAX_USERS}");

    // Testing Shadowing
    let a = 5;

    let a = a + 1; // Shadowing the first 'a'

    println!("The value of a is: {a}");

    {
        let a = a * 2; // Shadowing within new scope
        println!("The value of a in inner scope: {a}");
    }

    println!("The value of a outside scope is: {a}");

    let spaces = "       ";
    let spaces = spaces.len();

    println!("The spaces value is: {spaces}");
}
