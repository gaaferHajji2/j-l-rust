use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Pending {
    pub super_base: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };

        return Pending{ super_base: base }
    }
}