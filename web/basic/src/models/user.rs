use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}
