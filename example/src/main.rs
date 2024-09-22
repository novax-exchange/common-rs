use novax_tokio::tokio as tokio;
use novax_grpc;

mod http;
mod svc;

#[tokio::main]
async fn main() -> http::HttpResult<()>{
    #[cfg(feature = "grpc")]
    let _ = run_rpc().await;

    #[cfg(feature = "http")]
    let _ = http::main().await;

    Ok(())
}

#[allow(dead_code)]
async fn run_rpc() -> Result<(), Box<dyn std::error::Error>> {
    let (ctrlc_h, rx) = novax_tokio::ctrl_c_handler()?; 
    let addr = std::env::var("HTTP_ADDR").unwrap_or("[::1]:50055".to_string());
    let router = svc::sample_router();
    let rslt = novax_grpc::grpc_svc(addr, async move{
        let _ = rx.await;
    }, router).await;
    println!("grpc service terminate! {:?} ", rslt);
    let _ = ctrlc_h.await;
    Ok(())
}
