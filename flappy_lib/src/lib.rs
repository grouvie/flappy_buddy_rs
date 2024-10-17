mod error;
use error::BotResult;

use async_trait::async_trait;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{join, net::TcpStream, sync::mpsc};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[cfg(feature = "enable-tracing")]
use tracing::{error, info, trace};

#[async_trait]
pub trait FlappyConsumer {
    async fn handle_message(&mut self, message: String) -> String;
}

#[derive(Default)]
pub struct FlappyBot;

impl FlappyBot {
    pub async fn start<F>(&self, url: &str, mut consumer: F) -> BotResult<()>
    where
        F: FlappyConsumer + Send + 'static,
    {
        #[cfg(feature = "enable-tracing")]
        info!("Starting FlappyBot");

        let (message_sender, message_receiver) = mpsc::channel(64);
        let (consumer_sender, mut consumer_receiver) = mpsc::channel(64);

        tokio::spawn(async move {
            while let Some(message) = consumer_receiver.recv().await {
                #[cfg(feature = "enable-tracing")]
                trace!("Received message for handling: {message}");

                let response = consumer.handle_message(message).await;

                if let Err(error) = message_sender.send(response).await {
                    error!("{error}");
                };
            }
        });

        let socket_handler = SocketHandler::new(consumer_sender, message_receiver);
        if let Err(error) = start_socket(url, socket_handler).await {
            #[cfg(feature = "enable-tracing")]
            error!("Failed to start socket: {error}");
        }
        Ok(())
    }
}

struct SocketHandler {
    pub(crate) sender: mpsc::Sender<String>,
    pub(crate) receiver: mpsc::Receiver<String>,
}

impl SocketHandler {
    fn new(sender: mpsc::Sender<String>, receiver: mpsc::Receiver<String>) -> Self {
        Self { sender, receiver }
    }
}

async fn start_socket(url: &str, socket_handler: SocketHandler) -> BotResult<()> {
    #[cfg(feature = "enable-tracing")]
    info!("Connecting to WebSocket at: {url}");

    let (ws_stream, _) = connect_async(url).await?;

    let (write, read) = ws_stream.split();

    let join = tokio::spawn(async move {
        join!(
            read_from_socket(read, socket_handler.sender),
            write_to_socket(write, socket_handler.receiver)
        )
    });

    join.await?;
    Ok(())
}

async fn read_from_socket(
    mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    sender: mpsc::Sender<String>,
) {
    while let Some(Ok(message)) = read.next().await {
        match message {
            Message::Text(message) => {
                #[cfg(feature = "enable-tracing")]
                trace!("Received text message: {}", message);

                if let Err(error) = sender.send(message).await {
                    #[cfg(feature = "enable-tracing")]
                    error!("Failed to send message to channel: {error}");
                }
            }
            Message::Close(close_frame) => {
                #[cfg(feature = "enable-tracing")]
                info!("Received close frame: {close_frame:?}");
            }
            _ => {
                #[cfg(feature = "enable-tracing")]
                trace!("Received unhandled message type");
            }
        }
    }
}

async fn write_to_socket(
    mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut receiver: mpsc::Receiver<String>,
) {
    while let Some(message) = receiver.recv().await {
        #[cfg(feature = "enable-tracing")]
        trace!("Sending message to WebSocket: {}", message);

        if let Err(error) = write.send(Message::Text(message)).await {
            #[cfg(feature = "enable-tracing")]
            error!("Failed to send message over WebSocket: {error}");
        }
    }
}
