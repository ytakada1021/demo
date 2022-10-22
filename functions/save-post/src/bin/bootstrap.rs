use lambda_http::Error;
use save_post::handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_http::service_fn(handler::handler);

    lambda_http::run(handler).await?;

    Ok(())
}
