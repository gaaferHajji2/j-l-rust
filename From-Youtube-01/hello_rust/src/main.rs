fn main() {

    let y = 10;

    // y = 20; // This Will Raise Compile Error

    let mut x = 20;

    println!("The Value Of X, Before Changing Is: {}", x);

    x = 25; // If We Don't Use The Previous Value, Warning Will Appear

    println!("Hello, world!, y is: {}, x is: {}, x+y is: {}", y, x, y+x);


    /*
     * Testing The Variables Types
     */
    let a1 = 98_999;
    let a2 = 0xfff;
    let a3 = 0o337;
    let a4 = 0b1111_0000;
    let a5 = b'a';

    println!("a1 is: {}, a2 is: {}, a3 is: {}, a4 is: {}, a5 is: {}", a1, a2, a3, a4, a5)
}
