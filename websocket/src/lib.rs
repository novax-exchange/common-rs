use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{future, pin_mut, StreamExt};
use futures_channel::mpsc::{self, UnboundedSender};

type WsError =  tokio_tungstenite::tungstenite::error::Error;

/// fn ws_client provide and uri 
/// for websocket connection
/// 
pub async fn ws_client(uri: String, tx: tokio::sync::mpsc::Sender::<Vec<u8>> ) -> Result<UnboundedSender<Message>, WsError> {
    // for client use
    let (client_tx, client_rx) = mpsc::unbounded();
    let (ws_stream, _) = connect_async(&uri).await?;
    let (write, read) = ws_stream.split();
    tokio::spawn(async move {
        let ws_input_msg = client_rx.map(Ok).forward(write);
        let ws_ouput_msg = {
            read.for_each(|message| async {
                if let Ok(data) = message {
                    let tx = tx.clone();
                    tokio::spawn(async move {
                        let data = data.into_data();
                        let _ = tx.send(data).await;
                    });
                }
            })
        };
        pin_mut!(ws_input_msg, ws_ouput_msg);
        future::select(ws_input_msg, ws_ouput_msg).await;
    });
    Ok(client_tx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_ws_connect_to_test() {
        // wss from https://app.gosandy.io/
        let uri = "wss://ws.postman-echo.com/raw".to_string();
        let (tx, rx) = tokio::sync::mpsc::channel::<Vec::<u8>>(10);
        let skt = ws_client(uri, tx).await;
        assert_eq!(skt.is_ok(), true);
    }
}
