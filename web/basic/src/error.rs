use actix_web::{HttpResponse, ResponseError};
use std::{env, num};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] env::VarError),
    #[error("Failed to parse integer: {0}")]
    ParseInt(#[from] num::ParseIntError),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = json!({
            "error": {
                "type": match self {
                    AppError::DatabaseError(_) => "DATABASE_ERROR",
                    AppError::ValidationError(_) => "VALIDATION_ERROR",
                    AppError::ConfigError(_) => "CONFIG_ERROR",
                },
                "message": self.to_string()
            }
        });

        match self {
            AppError::ValidationError(_) => HttpResponse::BadRequest().json(error_response),
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().json(error_response),
            AppError::ConfigError(_) => HttpResponse::InternalServerError().json(error_response),
        }
    }
}
