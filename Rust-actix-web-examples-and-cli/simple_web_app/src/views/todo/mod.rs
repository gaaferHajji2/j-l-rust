mod create;
mod get;

use actix_web::web::{ServiceConfig, get, post, scope};

pub fn todo_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/todo")
            .route("/{title}", post().to(create::create))
            .route("/", get().to(get::get))
            
    );
}