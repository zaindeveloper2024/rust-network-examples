use actix_web::{middleware::from_fn, web, App, HttpServer};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use log;

mod handlers;
mod models;
mod middleware;
mod state;
mod error;
mod response;

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
            .service(handlers::health::hello)
            .service(handlers::health::db_test)
            .service(handlers::health::health)
            .route("/users", web::get().to(handlers::user::get_users))
            .route("/users", web::post().to(handlers::user::create_user))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
