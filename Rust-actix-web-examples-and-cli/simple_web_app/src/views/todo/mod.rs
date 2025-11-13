mod create;

use actix_web::web::{ServiceConfig, post, scope};

pub fn todo_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/todo")
            .route("/{title}", post().to(create::create))
            
    );
}