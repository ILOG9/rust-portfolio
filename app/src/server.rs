use crate::modules::{auth, chlabs};
use actix_web::{middleware::Logger, web, App, HttpServer};
use mongodb::Client;

#[actix_web::main]
pub async fn actix_web() -> std::io::Result<()> {
    // Database
    let uri: String =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client: Client = Client::with_uri_str(uri).await.expect("failed to connect");

    // Log backtrace
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Server
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(client.clone()))
            // Auth routes and controllers
            .service(auth::controller::controller::authorize)
            .service(auth::controller::controller::refresh_token)
            .service(auth::controller::controller::recover_account)
            .service(auth::controller::controller::validate_code_to_recover_account)
            .service(auth::controller::controller::change_password_after_recover_account)
            // CHlabs routes and controllers
            .service(chlabs::controller::controller::index)
            .service(chlabs::controller::controller::data_test)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
