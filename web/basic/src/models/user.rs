use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>, 
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100, message = "Invalid name length"))]
    pub name: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}
