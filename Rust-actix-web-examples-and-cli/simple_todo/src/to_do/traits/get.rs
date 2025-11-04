pub trait Get {
    fn get(&self, title: &str) {
        println!("Get todo with title: {}", title)
    }
}