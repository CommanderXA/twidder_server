use actix_web::HttpResponse;

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("Not found")
}
