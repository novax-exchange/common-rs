use tonic::transport::Certificate;
use std::error::Error;
use tonic::transport::server::Server;
use tonic::service::Routes;
use std::fs;
use tonic::transport::Identity;

pub struct KeyFiles {
    pub ca_pem: String, 
    pub svc_pem: String,
    pub svc_key: String,
}

pub async fn grpcs_svc<F>(
    srv_addr: String, 
    f: F,
    routes: Routes, 
    key_files: KeyFiles
) -> Result<(), Box::<dyn Error> > where F: core::future::Future<Output = ()>, 
{
    let addr: std::net::SocketAddr = srv_addr.parse()?;
    let ca_pem = fs::read(&key_files.ca_pem)?;
    let svc_cert = fs::read(&key_files.svc_pem)?;
    let svc_key = fs::read(&key_files.svc_key)?;
    let ident = Identity::from_pem(svc_cert, svc_key);
    let ca = Certificate::from_pem(ca_pem);
    
    let tls_cfg = tonic::transport::ServerTlsConfig::new()
        .identity(ident)
        .client_ca_root(ca);
    
    match Server::builder()
        .tls_config(tls_cfg)?
        .add_routes(routes)
        .serve_with_shutdown(addr, f).await 
    {
        Ok(_) => Ok(()),
        Err(e) => Err(  e.into() )
    }
}

pub use tonic;

#[cfg(test)]
mod test;

#[cfg(test)]
mod tests {
    use novax_tokio::tokio as tokio;

    #[tokio::test]
    async fn should_grpcs_service_start() -> Result<(), Box<dyn std::error::Error> >{
        let (ctrlc_h, rx) = novax_tokio::ctrl_c_handler()?; 
        let routes = super::test::sample_routes();
        let addr = "[::1]:50065".to_string();
        let rslt = super::grpcs_svc(addr, async move{
            let _ = rx.await;
        }, routes, super::KeyFiles{
            ca_pem: "certs/ca.pem".to_string(), 
            svc_pem: "certs/server.pem".to_string(),
            svc_key: "certs/server.key".to_string(),
        } ).await;
        // server start
        let _ = ctrlc_h.await;
        Ok(())
    }

}
