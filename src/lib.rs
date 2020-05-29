use actix_web::{Responder, HttpResponse};
use serde_json::json;

pub async fn index() -> impl Responder {
    let response = json!({
        "success": "true",
        "message": "Welcome home kid."
    });


    HttpResponse::Ok().body(response.to_string())
}
