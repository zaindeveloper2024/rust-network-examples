use crate::config;
use actix_cors::Cors;

pub fn setup_cors(config: &config::AppConfig) -> actix_cors::Cors {
    if config.cors_origin.contains(&"*".to_string()) {
        Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600)
    } else {
        Cors::default()
            .allowed_origin(config.cors_origin.as_str())
            .allow_any_method()
            .allow_any_header()
            .max_age(3600)
    }
}