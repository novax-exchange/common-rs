use tokio::sync::oneshot;
use tokio::task::JoinHandle;

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
