use actix_web::{middleware::{from_fn, Logger}, web, App, HttpServer};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use log;

mod handlers;
mod models;
mod middleware;
mod state;
mod error;
mod response;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new().expect("Failed to load configuration");

    // Initialize logger
    // env_logger::init_from_env(config.app.log_level.as_str());
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server in {:?} mode...", config.app.environment);

    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .acquire_timeout(std::time::Duration::from_secs(config.database.timeout_seconds))
        .connect(&config.database.url)
        .await
        .expect("Failed to create database pool");

    log::info!("Server is running at http://{}:{}", config.app.host, config.app.port);

    let config = web::Data::new(config);
    let state = web::Data::new(state::AppState { db: pool });

    let app_config = config.clone();


    HttpServer::new(move || {
        let cors = if app_config.app.cors_origin.contains(&"*".to_string()) {
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
        } else {
            Cors::default()
                .allowed_origin(app_config.app.cors_origin.as_str())
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
        };

        App::new()
            .wrap(cors)
            .wrap(from_fn(middleware::timer::time_middleware))
            .wrap(Logger::default())
            .app_data(app_config.clone())
            .app_data(state.clone())
            .service(handlers::health::hello)
            .service(handlers::health::db_test)
            .service(handlers::health::health)
            .route("/users", web::get().to(handlers::user::get_users))
            .route("/users", web::post().to(handlers::user::create_user))
    })
    .bind((config.app.host.clone(), config.app.port))?
    .run()
    .await
}
