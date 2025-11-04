mod state;

use std::env;
use state::{read_file, write_to_file};
use serde_json::value::Value;
use serde_json::{Map, json};

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = read_file("./jloka.json");
    state.insert(title.to_string(), json!(status));
    println!("The status is: {:?}", state);
    write_to_file("jloka.json", &mut state);
}