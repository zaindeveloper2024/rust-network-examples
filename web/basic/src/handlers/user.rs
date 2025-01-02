use crate::{
    error::AppError,
    models::user::{CreateUserRequest, User},
    state::AppState,
};
use actix_web::{web, Error, HttpResponse};
use validator::Validate;
use tracing::error;

pub async fn create_user(
    state: web::Data<AppState>,
    user: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    user.validate()?;

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

pub async fn get_users(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
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
