use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub message: T,
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn json_response<T: Serialize>(message: T) -> HttpResponse {
    let response = ApiResponse {
        message,
        status: String::from("OK"),
        timestamp: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(response)
}
