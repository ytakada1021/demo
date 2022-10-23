use lambda_http::Error;

pub mod service;
pub mod handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_http::service_fn(handler::handler);

    lambda_http::run(handler).await?;

    Ok(())
}
