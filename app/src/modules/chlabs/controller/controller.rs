use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse, Responder, Result,
};

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
struct IndexStruct {
    app_name: String,
    server_version: String,
    mail_contact: String,
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let obj = IndexStruct {
        app_name: dotenv!("APP_NAME").to_string(),
        server_version: dotenv!("APP_VERSION").to_string(),
        mail_contact: dotenv!("MAIL_CONTACT").to_string(),
    };
    Ok(Json(obj))
}

#[get("/data-test")]
async fn data_test() -> impl Responder {
    HttpResponse::Ok().body("Hello world2!")
}
