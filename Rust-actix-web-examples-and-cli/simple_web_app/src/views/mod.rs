mod auth;
mod todo;

use actix_web::web::ServiceConfig;

pub fn views_factory(app: &mut ServiceConfig) {
    auth::auth_factory(app);
    todo::todo_factory(app);
}