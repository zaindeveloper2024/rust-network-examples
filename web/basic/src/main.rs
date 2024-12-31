use actix_web::{
    body::MessageBody, dev::{ServiceRequest, ServiceResponse}, error::ErrorInternalServerError, get, middleware::{from_fn, DefaultHeaders, Logger, Next}, post, web, App, Error, HttpResponse, HttpServer, Responder
};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use sqlx::{pool, postgres::PgPoolOptions, Postgres,Pool};
use std::time::Instant;
use log;

struct AppState {
    db: Pool<Postgres>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

// pub struct Logger;

pub struct LoggerMiddleware<S> {
    service: S
}

#[derive(serde::Serialize)]
struct ApiResponse<T> {
    message: T,
    status: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}


async fn json_response<T: Serialize>(message: T) -> HttpResponse {
    let response = ApiResponse {
        message,
        status: String::from("OK"),
        timestamp: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(response)
}

async fn time_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let start = Instant::now();
    let response =next.call(req).await?;
    let duration = start.elapsed();
    println!("Request took: {:?}", duration);
    Ok(response)
}

#[get("/")]
async fn hello() -> impl Responder {
    json_response("Hello world!".to_string()).await
}

#[get("/health")]
async fn health() -> impl Responder {
    json_response("health".to_string()).await
}

#[get("/db-test")]
async fn db_test(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as::<_, (i64,)>("SELECT $1")
        .bind(1_i64)
        .fetch_one(&state.db)
        .await
        .map_err(ErrorInternalServerError)?;
    
    Ok(json_response(format!("Value from DB: {}", result.0)).await)
}

async fn get_users(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(json_response(users).await)
}

async fn create_user(
    state: web::Data<AppState>,
    user: web::Json<CreateUserRequest>,
) -> HttpResponse {
    match sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
    )
    .bind(&user.name)
    .bind(&user.email)
    .fetch_one(&state.db)
    .await
    {
        Ok(created_user) => HttpResponse::Created().json(created_user),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("This is an info log.");
    log::debug!("This is a debug log.");
    log::warn!("This is a warning log.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool");


    let state = web::Data::new(AppState { db: pool });

    let port = std::env::var("PORT")
        .map_err(|_| "PORT environment variable not set")
        .and_then(|p| p.parse::<u16>().map_err(|_| "PORT must be a valid number"))
        .unwrap_or(8080);

    println!("Server is running at http://127.0.0.1:{}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(from_fn(time_middleware))
            .app_data(state.clone())
            .service(hello)
            .service(db_test)
            .service(health)
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
