mod login;
mod logout;
mod profile;
mod signup;

use actix_web::web::{ServiceConfig, get, scope};

pub fn auth_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/auth")
            .route("login", get().to(login::login))
            .route("signup", get().to(signup::signup))
            .route("profile", get().to(profile::profile))
            .route("logout", get().to(logout::logout)),
    );
}