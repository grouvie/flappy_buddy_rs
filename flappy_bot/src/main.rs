use async_trait::async_trait;
use flappy_lib::{FlappyBot, FlappyConsumer};
use tracing::subscriber;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::FmtSubscriber;

struct MyFlappyConsumer;

#[async_trait]
impl FlappyConsumer for MyFlappyConsumer {
    async fn handle_message(&mut self, message: String) -> String {
        // Handle or modify the incoming message here

        tracing::debug!("{message:?}"); // Debug output for visibility

        message // return a message back out to the server
    }
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .finish();

    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let url = "wss://echo.websocket.org".to_owned();

    if let Err(error) = FlappyBot.start(url, MyFlappyConsumer).await {
        tracing::error!("{error}");
    };
}
