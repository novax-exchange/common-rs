use thiserror::Error;
use novax_http::axum as axum;
use novax_tokio::tokio as tokio;
// cors
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use serde_json::{Value, json};

use axum::{
    Extension, extract::DefaultBodyLimit,
    Json, Router, routing::get
};

async fn start_svc(rx: tokio::sync::oneshot::Receiver::<bool>) -> HttpResult<()> {
    let addr = std::env::var("HTTP_ADDR").unwrap_or("127.0.0.1:8086".to_string());
    let cors = CorsLayer::permissive().allow_origin(Any);
    let state = ("".to_string(), "".to_string());
    let general_route = Router::new().route("/version", get(version));
    let app = Router::new().merge(general_route)
        .layer(Extension(cors))
        .with_state( state.clone() )
        .layer(Extension(state))
        .layer(DefaultBodyLimit::max(10485760)); // 10 Mb /* the extra layer using for extenion grap */

    match novax_http::http_svc(app, addr, async move{
        let _ = rx.await;
        println!("novax http terminate");
    }).await {
        Ok(_) => {},
        Err(_) => {}
    };
    Ok(())
}

pub (crate) async fn main() -> HttpResult<()> {
    let (ctrlc_h, rx) = novax_tokio::ctrl_c_handler()?; 
    let runner_handle = tokio::spawn(async move{
        let _ = start_svc(rx).await;
    });
    let _ = ctrlc_h.await?;
    let _ = runner_handle.await?;
    Ok(())
}

const RESP_CODE: &str = "resp_code";
async fn version() -> Json::<Value> {
    Json(json!({"code":200, RESP_CODE:"0.0.1"}))
}

pub (crate) type HttpResult<T> = Result<T, HttpError>;

#[derive(Error, Debug)]
pub (crate) enum HttpError {
    #[error("Net address {0}")]
    NetAddrErr(#[from] std::net::AddrParseError),
    #[error("Standard error {0}")]
    StdErr(#[from] std::io::Error),
    #[error("Tokio join error {0}")]
    TokioJoinErr(#[from] tokio::task::JoinError),
}