pub trait Delete {
    fn delete(&self, title: &str) {
        println!("delete todo with title: {}", title);
    }
}