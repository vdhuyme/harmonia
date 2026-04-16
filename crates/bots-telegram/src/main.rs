use std::env;

use anyhow::Result;
use teloxide::{prelude::*, utils::command::BotCommands};
use reqwest::Client;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Supported commands:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "show the current queue for a room.")]
    Queue { room_id: String },
    #[command(description = "request a new song (requires URL).")]
    Request { room_id: String, url: String },
    #[command(description = "vote for a song by its ID.")]
    Vote { room_id: String, song_id: String },
}

async fn handle_command(cx: UpdateWithCx<Bot, Message>, cmd: Command) -> Result<()> {
    let api_base = env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".into());
    let client = Client::new();

    match cmd {
        Command::Help => {
            cx.answer(Command::descriptions()).await?;
        }
        Command::Queue { room_id } => {
            let url = format!("{}/queue/{}", api_base, room_id);
            let resp = client.get(&url).send().await?;
            let text = resp.text().await?;
            cx.answer(format!("Current queue:\\n{}", text)).await?;
        }
        Command::Request { room_id, url: song_url } => {
            let api_url = format!("{}/songs/request", api_base);
            let body = serde_json::json!({ "room_id": room_id, "url": song_url });
            let resp = client.post(&api_url).json(&body).send().await?;
            let text = resp.text().await?;
            cx.answer(format!("Request response: {}", text)).await?;
        }
        Command::Vote { room_id, song_id } => {
            let api_url = format!("{}/queue/{}/vote", api_base, room_id);
            let body = serde_json::json!({ "song_id": song_id });
            let resp = client.post(&api_url).json(&body).send().await?;
            let text = resp.text().await?;
            cx.answer(format!("Vote response: {}", text)).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Bot token from environment
    let bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let bot = Bot::new(bot_token).auto_send();

    // Dispatcher with command handling
    Dispatcher::builder(bot, Update::filter_message().branch(
        dptree::entry()
            .filter_command::<Command>()
            .endpoint(|cx, cmd| async move { handle_command(cx, cmd).await }),
    ))
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}