pub trait Edit {
    fn set_to_done(&self, title: &str) {
        println!("Set todo with title: {} to DONE", title);
    }

    fn set_to_pending(&self, title: &str) {
        println!("Set todo with title: {} to PENDING", title);
    }
}