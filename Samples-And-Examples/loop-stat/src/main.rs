fn main() {

    let mut count: i32 = 0;

    loop {
        count += 1;
        println!("The count is: {}", count);
        if count == 6 {
            break;
        }
    }

    let mut result = loop {
        count += 1;
        println!("The count is: {count}");

        if count > 6 {
            break count * 2;
        }
    };

    println!("The final result is: {result}");

    'outer: loop {
        
        loop {
            if result > 15 {
                break 'outer;
            }

            if result > 13 {
                break;
            }
        }

        result += 1;
        println!("The result is: {result}");
    }

    println!("Hello, world!");
}
