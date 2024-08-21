use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use std::error::Error;

pub async fn grpc_svc<F>(srv_addr: String, f: F) -> Result<(), Box::<dyn Error> > 
    where F: core::future::Future<Output = ()> 
{
    let addr: std::net::SocketAddr = srv_addr.parse()?;
    // todo!() <- this will hadle grpc services()
    Ok(())
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
