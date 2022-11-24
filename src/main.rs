use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let echo = event
        .query_string_parameters()
        .first("message")
        .unwrap_or("world!")
        .to_string();
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("<h1>Hello {}</h1>", echo).into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
