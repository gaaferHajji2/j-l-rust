use actix_web::{Responder, web};
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;
use crate::to_do::{ItemTypes, to_do_factory, enums::TaskStatus};
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn get() -> impl Responder {
    let state: Map<String, Value> = read_file("./jloka.json");

    let mut t1: Vec<ItemTypes> = Vec::new();

    for (key, value) in state {
         let status: TaskStatus = TaskStatus::from_string(value.to_string());
        let item: ItemTypes = to_do_factory(&key, status);
        t1.push(item);
    }

    let t2: ToDoItems = ToDoItems::new(t1);

    web::Json(t2)
}