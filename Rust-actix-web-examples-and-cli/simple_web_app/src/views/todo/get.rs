use actix_web::{Responder, web};
use serde_json::value::Value;
use serde_json::Map;

use crate::state::read_file;

pub async fn get() -> impl Responder {
    let state: Map<String, Value> = read_file("./jloka.json");

    web::Json(state)
}