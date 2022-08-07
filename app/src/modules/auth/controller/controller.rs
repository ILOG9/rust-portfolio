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

#[post("/api/auth/authorize")]
async fn authorize() -> impl Responder {
    HttpResponse::Ok().body("Hello world2!")
}

#[post("/api/auth/refresh-token")]
async fn refresh_token() -> impl Responder {
    HttpResponse::Ok().body("Hello world2!")
}

#[post("/api/auth/recover-account")]
async fn recover_account() -> impl Responder {
    HttpResponse::Ok().body("Hello world2!")
}

#[post("/api/auth/validate-code-to-recover-account")]
async fn validate_code_to_recover_account() -> impl Responder {
    HttpResponse::Ok().body("Hello world2!")
}

#[post("/api/auth/change-password-after-recover-account")]
async fn change_password_after_recover_account() -> impl Responder {
    HttpResponse::Ok().body("Hello world2!")
}
