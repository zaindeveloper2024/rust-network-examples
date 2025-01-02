use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    Error, middleware::Next,
};
use std::time::Instant;

pub async fn time_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let start = Instant::now();
    let response = next.call(req).await?;
    let duration = start.elapsed();
    log::info!("Request took: {:?}", duration);

    Ok(response)
}
