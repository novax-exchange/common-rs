mod svc;
pub mod error;

use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use error::{Error, Result};
use novax_log::log as log;

fn get_addr<'a>( lookup: &'a str, default_val: &'a str) -> String {
    // std::env::var(lookup).map_err(|_| default_val.to_string()).unwrap()
    std::env::var(lookup).unwrap_or(default_val.to_string())
}

//  runtime provide from invoker
pub async fn service(addr: String) -> Result<()> {
   
    #[cfg(feature = "grpc")]
    {
        let (ctrlc_h, rx) = ctrl_c_handler()?;
        let addr = if &addr == "" { get_addr("GRPC_ADDR", "[::1]:50055") } else { addr.clone() };
        let _ = svc::grpc_svc(addr, async move{
            // todo
            let _ = rx.await;
        }).await;
        let _ = ctrlc_h.await;
    }

    #[cfg(feature = "http")]
    {
        let (ctrlc_h, rx) = ctrl_c_handler()?;
        let addr = if &addr == "" { get_addr("HTTP_ADDR", "127.0.0.1:9091") } else { addr.clone() };
        let _ = svc::http_svc(addr, rx).await;
        println!(" http service shoud ne aex ");
        let _ = ctrlc_h.await;
    }
    // let fut_values = async {
    // };
    // fut_values.await;
    Ok(())
}


fn ctrl_c_handler() -> Result::<(JoinHandle::<bool>, oneshot::Receiver::<bool>)> {
    let (tx, rx) = oneshot::channel::<bool>();
    Ok (
        (
            tokio::task::spawn_blocking(
                move || {
                    let _ = tokio::spawn(async move{
                        tokio::signal::ctrl_c().await?;
                        let _ = tx.send(true);
                        log::info!(" ctrl + c receive ");
                        Ok::<(), Error>(())
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