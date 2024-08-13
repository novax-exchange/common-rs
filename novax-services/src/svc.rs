use crate::error::{Error, Result};
use tokio::sync::oneshot;
// http
use tokio::net::TcpListener;
use axum::{
    Extension, extract::DefaultBodyLimit,
    Json, Router, routing::get
};
use serde_json::{Value, json};
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
// http
use novax_log::log as log;

pub (crate) async fn grpc_svc<F>(srv_addr: String, /* DB pooling, */ f: F) -> Result<()> 
    where F: core::future::Future<Output = ()> 
{
    let addr: std::net::SocketAddr = srv_addr.parse()?;
    // todo!() <- this will hadle grpc services()
    Ok(())
}

// http restful service
pub (crate) async fn http_svc(addr: String, rx: oneshot::Receiver::<bool>) -> Result<()> {
    let addr = addr.parse::<std::net::SocketAddr>()?;
    let cors = CorsLayer::permissive().allow_origin(Any);
    
    let state = ("".to_string(), "".to_string());
    let general_route = 
        Router::new()
        .route("/version", get(version));

    let app = Router::new().merge(general_route)
        .layer(Extension(cors))
        .with_state( state.clone() )
        .layer(Extension(state))
        .layer(DefaultBodyLimit::max(10485760)); // 10 Mb /* the extra layer using for extenion grap */

    log::info!("listening on {}", addr);
    let listener = TcpListener::bind(&addr).await?;
    let rslt = match axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async move {
            let _ = rx.await;
            log::info!("server gracefully terminate");
        })
        .await 
    {
        Ok(rslt) => Ok( rslt ),
        Err(e) => {
            Err(Error::StrMsgErr(format!("internal hyper error {:?}", e)))
        }
    };
    rslt
}
const RESP_CODE: &str = "resp_code";
async fn version() -> Json::<Value> {
    Json(json!({"code":200, RESP_CODE:"0.0.1"}))
}
