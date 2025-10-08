fn main() {

    // immutable array with values
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

    // get slice of array from 1 to 4 (Note: array start from 0)
    let arr_slice = &arr[1..5];
    println!("The array slice is: {:?}", arr_slice);
    println!("----------------------------------");

    println!("Hello, world!");
    println!("----------------------------------");

    // mutable array with values
    let mut arr2:[i32; 5] = [0, 1, 2, 3, 4];
    arr2[0] = 500000;
    println!("The array is: {:?}", arr2);
}
