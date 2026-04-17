use application::QueueService;
use domain::error::DomainResult;
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Deserialize)]
struct TelegramUpdate {
    message: Option<TelegramMessage>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct TelegramMessage {
    text: Option<String>,
    chat: Chat,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Chat {
    id: i64,
}

struct TelegramBot {
    #[allow(dead_code)]
    api_client: Client,
    #[allow(dead_code)]
    token: String,
    #[allow(dead_code)]
    api_url: String,
    #[allow(dead_code)]
    queue_service: Arc<QueueService>,
}

impl TelegramBot {
    pub fn new(token: String, queue_service: Arc<QueueService>) -> Self {
        Self {
            api_client: Client::new(),
            token,
            api_url: "https://api.telegram.org/bot".to_string(),
            queue_service,
        }
    }

    #[allow(dead_code)]
    async fn handle_message(
        &self,
        message: TelegramMessage,
    ) -> DomainResult<()> {
        if let Some(text) = message.text {
            if text.starts_with("/play") {
                // Simplified: /play <track_id> <provider>
                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.len() >= 3 {
                    tracing::info!(
                        "Telegram request to play track: {}",
                        parts[1]
                    );
                    // In a real implementation, we would call queue_service.add_song
                }
            } else if text == "/queue" {
                tracing::info!("Telegram request to see queue");
                // In a real implementation, we would call queue_service.get_sorted_queue
            }
        }
        Ok(())
    }

    pub async fn run(&self) {
        tracing::info!("Telegram bot started");
        // In a real implementation, we would use long polling or webhooks
        // to receive updates from the Telegram API
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // These would come from environment variables
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .unwrap_or_else(|_| "placeholder".to_string());

    // Mocking dependencies for the bot
    // In reality, these would be initialized similar to the API
    let redis = infrastructure::RedisClient::new("redis://127.0.0.1").unwrap();
    let db = sea_orm::Database::connect(
        "postgres://user:password@localhost/harmonia",
    )
    .await
    .unwrap();
    let repo = Arc::new(infrastructure::SqlRepository::new(db));
    let lock_manager =
        Arc::new(infrastructure::RedisLockManager::new(Arc::new(redis)));
    let queue_service = Arc::new(QueueService::new(repo, lock_manager));

    let bot = TelegramBot::new(token, queue_service);
    bot.run().await;
}
