pub trait Create{
    fn create(&self, title: &str) {
        println!("create new todo with title: {}", title);
    }
}