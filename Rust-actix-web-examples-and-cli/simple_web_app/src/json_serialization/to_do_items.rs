use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;
use serde::Serialize;

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
}
