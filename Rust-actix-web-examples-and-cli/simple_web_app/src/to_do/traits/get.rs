use serde_json::Map;
use serde_json::value::Value;

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(value) => {
                println!("The title is: {}", title);
                println!("the status is: {}\n", value);
            }
            None => println!("Todo with title {} not found", title)
        }
    }
}