use actix_web::{
    error::ErrorInternalServerError,
    get,
    web, Error, HttpResponse, Responder
 };
 
 use crate::{
    response,
    state,
 };

 #[get("/")]
 pub async fn hello() -> impl Responder {
    response::json_response("Hello world!".to_string()).await
 }
 
 #[get("/health")]
 pub async fn health() -> impl Responder {
    response::json_response("health".to_string()).await
 }
 
 #[get("/db-test")]
 pub async fn db_test(state: web::Data<state::AppState>) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as::<_, (i64,)>("SELECT $1")
         .bind(1_i64)
         .fetch_one(&state.db)
         .await
         .map_err(ErrorInternalServerError)?;
     
    Ok(response::json_response(format!("Value from DB: {}", result.0)).await)
 }
 