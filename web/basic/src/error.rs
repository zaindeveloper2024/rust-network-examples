use actix_web::{HttpResponse, ResponseError};
use std::{fmt, env, num};
use thiserror::Error;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    ValidationError(validator::ValidationErrors),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] env::VarError),
    #[error("Failed to parse integer: {0}")]
    ParseInt(#[from] num::ParseIntError),
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        AppError::ValidationError(errors)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ValidationError(e) => HttpResponse::BadRequest().json(e),
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
        }
    }
}