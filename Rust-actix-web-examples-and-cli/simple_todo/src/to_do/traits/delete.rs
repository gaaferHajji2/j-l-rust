use serde_json::Map;
use serde_json::value::Value;

use crate::state::write_to_file;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);
        write_to_file("./jloka.json", state);
        println!("todo with title: {} deleted", title);
    }
}