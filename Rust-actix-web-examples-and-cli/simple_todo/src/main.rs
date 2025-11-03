mod to_do;

fn main() {
    println!("Hello, world!");
    println!("Status-01: {}", to_do::enums::TaskStatus::DONE);
    println!("Status-02: {}", to_do::enums::TaskStatus::PENDING);
}
