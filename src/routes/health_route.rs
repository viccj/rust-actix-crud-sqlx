use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/api/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build simple CRUD";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}
