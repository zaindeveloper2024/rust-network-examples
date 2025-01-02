use actix_web::{middleware::{from_fn, Logger}, web, App, HttpServer};
use log;

mod handlers;
mod models;
mod middleware;
mod state;
mod error;
mod response;
mod config;
mod database;
mod cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new().expect("Failed to load configuration");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server in {:?} mode...", config.app.environment);

    let pool = database::setup_database(&config.database).await.expect("Failed to create database pool");

    log::info!("Server is running at http://{}:{}", config.app.host, config.app.port);

    let config = web::Data::new(config);
    let state = web::Data::new(state::AppState { db: pool });

    let app_config = config.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(cors::setup_cors(&app_config.app))
            .wrap(from_fn(middleware::timer::time_middleware))
            .wrap(Logger::default())
            .app_data(app_config.clone())
            .app_data(state.clone())
            .configure(configure_routes)
    })
    .bind((config.app.host.clone(), config.app.port))?
    .run()
    .await
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::health::hello)
        .service(handlers::health::db_test)
        .service(handlers::health::health)
        .route("/users", web::get().to(handlers::user::get_users))
        .route("/users", web::post().to(handlers::user::create_user));
}