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
}
