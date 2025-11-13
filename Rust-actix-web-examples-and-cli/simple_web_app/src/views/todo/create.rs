use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::state::{self, read_file};
use crate::operation::process_input;

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./jloka.json");
    let title: String = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);

    format!("todo with title: {}, created", title)
}