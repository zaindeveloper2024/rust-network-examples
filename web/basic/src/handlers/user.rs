use crate::{
    error::AppError,
    models::user::{CreateUserRequest, User},
    state::AppState,
};
use actix_web::{web, HttpResponse};
use serde_json::json;
use validator::Validate;
use tracing::error;

pub async fn create_user(
    state: web::Data<AppState>,
    user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    user.validate()?;

    let exists = check_user_existence(state.clone(), &user.email).await?;
    if exists {
        error!("User already exists");
        return Ok(HttpResponse::Conflict().json(json!({
            "error": {
                "code": "conflict",
                "message": "User already exists"
            }
        })))
    }

    let created_user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
    )
    .bind(&user.name)
    .bind(&user.email)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to execute query: {:?}", e);
        AppError::DatabaseError(e)
    })?;

    Ok(HttpResponse::Created().json(created_user))
}

pub async fn get_users(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users"
    )
    .fetch_all(&state.db)
   .await
   .map_err(|e| {
        error!("Failed to execute query: {:?}", e);
        AppError::DatabaseError(e)
    })?;

    Ok(HttpResponse::Ok().json(users))
}

async fn check_user_existence(state: web::Data<AppState>, email: &str) -> Result<bool, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to execute query: {:?}", e);
        AppError::DatabaseError(e)
    })?;

    Ok(user.is_some())
}
