use novax_http::axum as axum;
use axum_server::tls_rustls::RustlsConfig;
use axum::Router;
use std::error::Error;
use std::net::SocketAddr;
use std::env;
use std::path::Path;

pub async fn https_svc(app: Router, addr: String) -> Result<(), Box::<dyn Error>> {
    // run https server
    let addr = addr.parse::<SocketAddr>()?;
    let cfg_pem = config_pem();
    let config = RustlsConfig::from_pem_file(
        Path::new(&cfg_pem.0), Path::new(&cfg_pem.1)
    ).await?;
    axum_server::bind_rustls(addr, config)
        // .handle(handle)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

fn config_pem() -> (String, String) {
    (
        env::var("CERT_FILE").unwrap_or("key/client_cert.pem".to_string()),
        env::var("KEY_FILE").unwrap_or("key/client_key.pem".to_string()),
    )
}

pub use axum_server;


#[cfg(test)]
mod test {
    use super::*;
    use novax_tokio::tokio as tokio;
    use novax_http::axum::{routing::get, Router};
    
    #[tokio::test]
    async fn should_tls_server_start() -> Result<(), Box<dyn Error>> {
        let app = Router::new().route("/", get(handler));
        let rslt = super::https_svc(app, "127.0.0.1:9092".to_string()).await?;
        Ok(rslt)
    }

    async fn handler<'a>() -> &'a str {
        "test"
    }
}
