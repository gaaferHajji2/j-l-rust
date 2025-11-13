pub mod structs;
pub mod enums;
pub mod traits;

use std::fmt;
use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => { ItemTypes::Done(Done::new(title)) },
        TaskStatus::PENDING => { ItemTypes::Pending(Pending::new(title)) }
    }
}

impl fmt::Display for ItemTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::Done(t1) => { write!(f, "Title is: {}, status is: {}", t1.super_struct.title, t1.super_struct.status) },
            &Self::Pending(t1) => { write!(f, "Title is: {}, status is: {}", t1.super_base.title, t1.super_base.status) }
        }
    }
}