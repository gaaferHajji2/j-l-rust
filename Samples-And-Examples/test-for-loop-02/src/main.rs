fn main() {

    let mut arr = [0, 1, 2, 3, 4, 5];
    for num in arr.iter_mut() {
        *num*=2;
    }

    println!("The doubled array is: {:?}", arr);
    println!("Hello, world!");
}
