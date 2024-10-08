use async_trait::async_trait;
use flappy_lib::{FlappyBot, FlappyConsumer};
use tokio::sync::mpsc;
use tracing::subscriber;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::FmtSubscriber;

struct MyFlappyConsumer;

#[async_trait]
impl FlappyConsumer for MyFlappyConsumer {
    async fn handle_message(&mut self, message: String, sender: mpsc::Sender<String>) {
        let data = message;

        tracing::debug!("{data:?}");

        if let Err(error) = sender.send(data).await {
            tracing::error!("{error}");
        };
    }
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .finish();

    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let url = "wss://echo.websocket.org".to_owned();

    let consumer = MyFlappyConsumer;

    if let Err(error) = FlappyBot.start(url, consumer).await {
        tracing::error!("{error}");
    };
}
