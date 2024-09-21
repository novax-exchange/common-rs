use novax_http::axum as axum;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io::{Result as IoRslt, Error as IoError, BufReader};
use rustls::pki_types::CertificateDer;
use std::sync::Arc;
use std::error::Error;
use std::net::SocketAddr;

type CertItem = Result<CertificateDer<'static>, IoError>;

fn cert_loader(s: &'_ str) -> IoRslt<Vec::<CertItem>> {
    let mut buf_r = BufReader::new( File::open(s)? );
    Ok( certs(&mut buf_r).collect::<Vec::<CertItem>>() )
}

// loacation of keys files
pub struct KeyFile {
    pub verified_client: bool,
    pub ca_crt: String,
    pub host_crt: String,
    pub host_key: String,
}

pub async fn https_svc(app: Router, addr: String, key_file: KeyFile) -> Result<(), Box::<dyn Error>> {
    let addr: SocketAddr = addr.parse::<SocketAddr>()?;
    let host_crt =  key_file.host_crt.to_string();
    let mut ca =  cert_loader(&key_file.ca_crt)?;
    let mut host = cert_loader(&host_crt[..])?;
    // server key
    let svc_key = private_key(    
        &mut BufReader::new(File::open(&key_file.host_key)?)
    )?.unwrap();
    // tls
    let mut root_cert = rustls::RootCertStore::empty();
    let ca_item = ca.pop();
    let ca_item = ca_item.unwrap();
    // rootstore
    let _ = root_cert.add(ca_item?);
    let client_verifier = rustls::server::WebPkiClientVerifier::builder(root_cert.into()).build()?;
    let cert_der = host.pop().unwrap();
    let cfg = 
        if key_file.verified_client {
            rustls::ServerConfig::builder()
            .with_client_cert_verifier(client_verifier)
            .with_single_cert(vec![cert_der?], svc_key )?
        } else {
            rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![cert_der?], svc_key )?
        };
    let config = RustlsConfig::from_config(Arc::new(cfg));
    
    let _ = axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;
    Ok(())
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
        let key_file = super::KeyFile {
            verified_client: true,
            ca_crt: "certs/localhost/ca.crt".to_string(),
            host_crt: "certs/localhost/localhost.crt".to_string(),
            host_key: "certs/localhost/localhost.key".to_string(),
        };
        let rslt = super::https_svc(app, "127.0.0.1:9092".to_string(), key_file).await?;
        Ok(rslt)
    }

    async fn handler<'a>() -> &'a str {
        "test"
    }
}
