use novax_tokio::tokio as tokio;
use novax_grpc;

mod http;


#[tokio::main]
async fn main() -> http::HttpResult<()>{
    // TODO!
    // 1. add new proto file and compile with protoc or using build.rs
    // 2. hook up with the serviec(1) and start using gRpc to communicate among services 
    let addr = std::env::var("HTTP_ADDR").unwrap_or("[::1]:50055".to_string());
    let _ = novax_grpc::grpc_svc(addr, async move{
        // todo
        // let _ = rx.await;
    }).await;
    // http
    let _ = http::main().await;
    Ok(())
}

