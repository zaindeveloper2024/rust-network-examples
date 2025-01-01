use actix_web::{
   error::ErrorInternalServerError,
   get,
   middleware::from_fn, web, App, Error, HttpResponse, HttpServer, Responder
};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use log;

mod handlers;
mod models;
mod state;
mod middleware;
mod error;
mod response;

#[get("/")]
async fn hello() -> impl Responder {
    response::json_response("Hello world!".to_string()).await
}

#[get("/health")]
async fn health() -> impl Responder {
    response::json_response("health".to_string()).await
}

#[get("/db-test")]
async fn db_test(state: web::Data<state::AppState>) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as::<_, (i64,)>("SELECT $1")
        .bind(1_i64)
        .fetch_one(&state.db)
        .await
        .map_err(ErrorInternalServerError)?;
    
    Ok(response::json_response(format!("Value from DB: {}", result.0)).await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool");

    let port = std::env::var("PORT")
        .map_err(|_| "PORT environment variable not set")
        .and_then(|p| p.parse::<u16>().map_err(|_| "PORT must be a valid number"))
        .unwrap_or(8080);

    println!("Server is running at http://127.0.0.1:{}", port);

    let state = web::Data::new(state::AppState { db: pool });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(from_fn(middleware::timer::time_middleware))
            .app_data(state.clone())
            .service(hello)
            .service(db_test)
            .service(health)
            .route("/users", web::get().to(handlers::user::get_users))
            .route("/users", web::post().to(handlers::user::create_user))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
