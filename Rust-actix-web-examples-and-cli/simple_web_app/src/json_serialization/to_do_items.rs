use crate::state::{read_file};
use crate::to_do::{ItemTypes, to_do_factory};
use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::base::Base;
use actix_web::http::header::ContentType;
use actix_web::{Responder, HttpRequest, HttpResponse};
use actix_web::body::BoxBody;
use serde::Serialize;
use serde_json::{Map, value::Value};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array: Vec<Base> = Vec::new();
        let mut done_array: Vec<Base> = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(t1) => pending_array.push(t1.super_base),
                ItemTypes::Done(t1) => done_array.push(t1.super_struct),
            }
        }

        let done_count: i8 = done_array.len() as i8;
        let pending_count: i8 = pending_array.len() as i8;

        return ToDoItems {
            pending_items: pending_array,
            done_items: done_array,
            pending_item_count: pending_count,
            done_item_count: done_count,
        };
    }

    pub fn get_state() -> ToDoItems {
        let state: Map<String, Value> = read_file("./jloka.json");
        let mut t1: Vec<_> = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
            let t2 = to_do_factory(&key, status);
            t1.push(t2);
        }

        return ToDoItems::new(t1);
        
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let t1 = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json()).body(t1)
    }
}