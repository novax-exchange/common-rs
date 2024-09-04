use novax_http::tokio as tokio;
use novax_grpc;

mod http;


#[tokio::main]
async fn main() -> http::HttpResult<()>{
    // TODO!
    // grpc more throughly example
    let addr = std::env::var("HTTP_ADDR").unwrap_or("[::1]:50055".to_string());
    let _ = novax_grpc::grpc_svc(addr, async move{
        // todo
        // let _ = rx.await;
    }).await;
    // http
    let _ = http::main().await;
    Ok(())
}

