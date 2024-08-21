use axum::Router;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use std::error::Error;
use tokio::net::TcpListener;
use core::future::Future;

pub async fn http_svc<F>(app: Router, addr: String, f: F) -> Result<(), Box::<dyn Error> > 
    where F:  Future<Output = ()> + Send + 'static
{
    let addr = addr.parse::<std::net::SocketAddr>()?;
    let listener = TcpListener::bind(&addr).await?;
    let rslt = match axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(f)
        .await 
    {
        Ok(rslt) => Ok( rslt ),
        Err(e) => Err(e.into())
    };
    rslt
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
pub use axum;
