use actix_web::{middleware::Logger, App, HttpServer};

use crate::modules::{auth, chlabs};

#[actix_web::main]
pub async fn actix_web() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            // Auth routes and controllers
            .service(auth::controller::controller::authorize)
            .service(auth::controller::controller::refresh_token)
            .service(auth::controller::controller::recover_account)
            .service(auth::controller::controller::validate_code_to_recover_account)
            .service(auth::controller::controller::change_password_after_recover_account)
            // CHlabs routes and controllers
            .service(chlabs::controller::controller::index)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
