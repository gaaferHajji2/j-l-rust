mod to_do;
// use to_do::structs::pending::Pending;
// use to_do::structs::done::Done;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use to_do::ItemTypes;

use crate::to_do::traits::get::Get;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::delete::Delete;

fn main() {
    println!("Hello, world!");
    // Check the status
    // println!("Status-01: {}", to_do::enums::TaskStatus::DONE);
    // println!("Status-02: {}", to_do::enums::TaskStatus::PENDING);

    // Check Pending
    // let t1 = Pending::new("JLoka-01");
    // println!("t1 title is: {}", t1.super_base.title);
    // println!("t1 status is: {}", t1.super_base.status);

    // Check Done
    // let t2 = Done::new("JLoka-02");
    // println!("t2 title is: {}", t2.super_struct.title);
    // println!("t2 status is: {}", t2.super_struct.status);

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
