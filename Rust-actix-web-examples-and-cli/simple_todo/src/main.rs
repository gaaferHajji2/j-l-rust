mod to_do;
mod state;
mod operation;

use std::os::windows::process;
use std::{env, result};

use serde_json::Map;
use serde_json::value::Value;

use state::read_file;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use operation::process_input;

fn main() {
    println!("Hello, world!");

    // let t1: ItemTypes = to_do_factory("JLoka-01", TaskStatus::DONE);
    // println!("t1 is: {}", t1);
    // let t1: ItemTypes = to_do_factory("JLoka-02", TaskStatus::PENDING);
    // println!("t2 is: {}", t1);

    // match t1 {
    //     ItemTypes::Done(t2) => { 
    //         t2.get(&t2.super_struct.title);
    //         t2.delete(&t2.super_struct.title);
    //     }

    //     ItemTypes::Pending(t2) => {
    //         t2.get(&t2.super_base.title);
    //         t2.set_to_done(&t2.super_base.title);
    //     }
    // }

    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file("./jloka.json");
    let status: String;

    match &state.get(*&title) {
        Some(t1) => {
            status = t1.to_string().replace("\"", "");
        }
        None => {
            status = "pending".to_owned();
        }
    }

    let item = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));

    process_input(item, command.to_string(), &state);
}
