fn main() {

    let y = 10;

    // y = 20; // This Will Raise Compile Error

    let mut x = 20;

    println!("The Value Of X, Before Changing Is: {}", x);

    x = 25; // If We Don't Use The Previous Value, Warning Will Appear

    println!("Hello, world!, y is: {}, x is: {}, x+y is: {}", y, x, y+x);


}
