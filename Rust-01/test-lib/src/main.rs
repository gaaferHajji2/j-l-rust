use num_cpus;
fn main() {
    println!("The num of cpus is: {}", num_cpus::get());
    println!("The num of physical cpu is: {}", num_cpus::get_physical())
}