use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::extract::ws::{Message, WebSocket};
use redis::aio::PubSub;
use tokio::sync::broadcast;

use crate::state::AppState;
use crate::state::AppState;

/// Register the websocket route (to be called from the main API router)
pub fn router() -> Router<AppState> {
    Router::new().route(
        "/ws/:room_id",
        get(ws_handler),
    )
}

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // In a real implementation, you would extract authentication info here.
    // For this scaffold we accept all connections.
    ws.on_upgrade(move |socket| handle_socket(socket, room_id, state))
}

/// Core socket handling logic
async fn handle_socket(mut socket: WebSocket, room_id: String, state: AppState) {
    // 1. Create a new PubSub connection for this WebSocket session
    let mut pubsub = match state.redis.get_async_connection().await {
        Ok(mut conn) => match conn.as_pubsub().await {
            Ok(p) => p,
            Err(e) => {
                let _ = socket
                    .send(Message::Text(format!("pubsub error: {}", e)))
                    .await;
                return;
            }
        },
        Err(e) => {
            let _ = socket
                .send(Message::Text(format!("redis error: {}", e)))
                .await;
            return;
        }
    };

    // 2. Subscribe to the room channel
    let channel_name = format!("room:{}", room_id);
    if let Err(e) = pubsub.subscribe(&channel_name).await {
        let _ = socket
            .send(Message::Text(format!("subscribe error: {}", e)))
            .await;
        return;
    }

    // 3. Send initial empty queue state (placeholder)
    let initial_state = serde_json::json!({ "room": room_id, "queue": [] });
    if socket
        .send(Message::Text(initial_state.to_string()))
        .await
        .is_err()
    {
        return;
    }

    // 4. Forward messages from Redis PubSub to the WebSocket client
    while let Ok(msg) = pubsub.on_message().await {
        let payload = msg.get_payload::<String>().unwrap_or_default();
        if socket.send(Message::Text(payload)).await.is_err() {
            break;
        }
    }

    // Connection closed – cleanup happens automatically.
}