use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use std::error::Error;
use tonic::transport::server::Router;


pub async fn grpc_svc<F>(srv_addr: String, f: F, 
    router: Router) -> Result<(), Box::<dyn Error> > 
    where F: core::future::Future<Output = ()>, 
{
    let addr: std::net::SocketAddr = srv_addr.parse()?;
    match router.serve_with_shutdown(addr, f).await {
        Ok(_) => Ok(()),
        Err(e) => Err(  e.into() )
    }
}

pub fn ctrl_c_handler() -> Result::<(JoinHandle::<bool>, oneshot::Receiver::<bool>), std::io::Error> {
    let (tx, rx) = oneshot::channel::<bool>();
    Ok (
        (
            tokio::task::spawn_blocking(
                move || {
                    let _ = tokio::spawn(async move{
                        tokio::signal::ctrl_c().await?;
                        let _ = tx.send(true);
                        Ok::<(), std::io::Error>(())
                    });
                    true
                }
            ),
            rx
        )
    )
}

// re-export
pub use tokio;
pub use tonic;
