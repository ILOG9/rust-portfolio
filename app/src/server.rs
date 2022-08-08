use crate::modules::{auth, chlabs};
use actix_web::{middleware::Logger, web, App, HttpServer};
use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};

#[actix_web::main]
pub async fn actix_web() -> std::io::Result<()> {
    // Database
    let uri: String =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client: Client = Client::with_uri_str(uri).await.expect("failed to connect");

    // Add database rules
    index_fields_in_database(&client, "user", "email").await;

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
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}

async fn index_fields_in_database(client: &Client, doc: &str, field: &str) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { field: 1 })
        .options(options)
        .build();
    client
        .database(dotenv!("DB_NAME"))
        .collection::<auth::models::user::User>(doc)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}
