mod to_do;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use to_do::ItemTypes;

use crate::to_do::traits::get::Get;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::delete::Delete;

fn main() {
    println!("Hello, world!");

    let t1: ItemTypes = to_do_factory("JLoka-01", TaskStatus::DONE);
    println!("t1 is: {}", t1);
    let t1: ItemTypes = to_do_factory("JLoka-02", TaskStatus::PENDING);
    println!("t2 is: {}", t1);

    match t1 {
        ItemTypes::Done(t2) => { 
            t2.get(&t2.super_struct.title);
            t2.delete(&t2.super_struct.title);
        }

        ItemTypes::Pending(t2) => {
            t2.get(&t2.super_base.title);
            t2.set_to_done(&t2.super_base.title);
        }
    }
}
