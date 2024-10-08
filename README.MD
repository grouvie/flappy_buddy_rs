# Flappy Buddy Rust Client

Welcome to **Flappy Buddy.rs**! This project offers a streamlined WebSocket client in Rust for the Coding Buddies Challenge, helping you quickly set up and handle WebSocket messaging in an async Rust environment.

## Getting Started

To dive in:
1. Head over to [flappy_bot/src/main.rs](flappy_bot/src/main.rs) to see a **basic setup**. This example shows you how to:
    - Implement the [FlappyConsumer](flappy_lib/src/lib.rs#L16) trait on a struct.
    - Start a WebSocket client with `FlappyBot`, connecting to a game server and echoing back any received messages.

2. **Run the Example Bot**: You can test the echo bot immediately by running:
    ```bash
    cargo run --bin flappy_bot
    ```

3. **Customize the example**:
   - You can **replace** the provided [MyFlappyConsumer](flappy_bot/src/main.rs#L8) struct or define your own that implements the [FlappyConsumer](flappy_lib/src/lib.rs#L16) trait.
   - To get started, specify the game server's WebSocket URL, customize the message handling logic in your consumer, and call the `start` method with your URL and consumer instance in the `main` function to establish the connection.

### Basic Echo Example:

Here’s the complete example from `main.rs`, demonstrating how to handle messages:

```rust
use async_trait::async_trait;
use flappy_lib::{FlappyBot, FlappyConsumer};
use tokio::sync::mpsc;
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
```

Happy coding with **Flappy Buddy.rs**!