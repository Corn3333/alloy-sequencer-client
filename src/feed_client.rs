use crate::errors::{ConnectionUpdate, RelayError};
use crate::types::Root;
use futures::StreamExt;
use log::*;
use tokio::{
    net::TcpStream,
    task::JoinHandle,
    sync::mpsc::Sender,
    time::{sleep, Duration},
};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

/// Sequencer Feed Client
pub struct RelayClient {
    // Socket connection to read from
    connection: WebSocketStream<MaybeTlsStream<TcpStream>>,
    // For sending errors / disconnects
    connection_update: Sender<ConnectionUpdate>,
    // Sends Transactions
    sender: Sender<Root>,
    // Relay ID
    id: u32,
}

impl RelayClient {
    // Does not start the reader, only makes the websocket connection
    pub async fn new(
        url: Url,
        // chain_id: u64,
        id: u32,
        sender: Sender<Root>,
        connection_update: Sender<ConnectionUpdate>,
    ) -> Result<Self, RelayError> {
        info!("Adding client | Client Id: {}", id);

        let key = tungstenite::handshake::client::generate_key();
        let host = url
            .host_str()
            .ok_or(RelayError::InvalidUrl)?;

        let req = tungstenite::handshake::client::Request::builder()
            .method("GET")
            .uri(url.as_str())
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", key)
            .header("Arbitrum-Feed-Client-Version", "2")
            .header("Arbitrum-Requested-Sequence-number", "0")
            .body(())?;

        let (socket, _resp) = connect_async(req).await?;
        
        Ok(Self {
            connection: socket,
            connection_update,
            sender,
            id,
        })
    }

    // Start the reader
    pub fn spawn(self) -> JoinHandle<()> {
        info!("Sequencer feed reader started | Client Id: {}", self.id);

        tokio::spawn(async move {
            match self.run().await {
                Ok(_) => (),
                Err(e) => error!("{}", e),
            }
        })
    }

    pub async fn run(mut self) -> Result<(), RelayError> {
        while let Some(msg) = self.connection.next().await {
            match msg {
                Ok(message) => {
                    let decoded_root: Root = match serde_json::from_slice(&message.into_data()) {
                        Ok(d) => d,
                        Err(_) => {                            
                            sleep(Duration::from_millis(10)).await;
                            continue
                        },
                    };

                    if self.sender.send(decoded_root).await.is_err() {
                        break; // we gracefully exit
                    }
                }
                Err(e) => {
                    self.connection_update
                        .send(ConnectionUpdate::StoppedSendingFrames(self.id))
                        .await?;
                    error!("Connection closed with error: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
