fn main() {

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    for num in arr {
        println!("The num is: {num}");
    }
    println!("----------------------------------");

    for i in 1..6 {
        println!("i is: {i}");
    }
    println!("----------------------------------");

    for num in arr.iter() {
        println!("The num is: {}", num)
    }
    println!("----------------------------------");

    println!("Hello, world!");
}
