use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::StreamExt;

use crate::state::AppState;

/// Register the websocket route (to be called from the main API router)
pub fn router() -> Router<AppState> {
    Router::new().route("/ws/:room_id", get(ws_handler))
}

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, room_id, state))
}

/// Core socket handling logic
async fn handle_socket(
    mut socket: WebSocket,
    room_id: String,
    state: AppState,
) {
    let mut pubsub_conn = match state.redis.get_async_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            let _ = socket
                .send(Message::Text(format!("redis error: {}", e)))
                .await;
            return;
        }
    };

    // Fixed: into_pubsub() in redis-rs aio returns the PubSub object directly, not a Result
    let mut pubsub = pubsub_conn.into_pubsub();

    let channel_name = format!("room:{}", room_id);
    if let Err(e) = pubsub.subscribe(&channel_name).await {
        let _ = socket
            .send(Message::Text(format!("subscribe error: {}", e)))
            .await;
        return;
    }

    let initial_state = serde_json::json!({ "room": room_id, "queue": [] });
    if socket
        .send(Message::Text(initial_state.to_string()))
        .await
        .is_err()
    {
        return;
    }

    let mut pubsub_stream = pubsub.on_message();
    while let Some(msg) = pubsub_stream.next().await {
        // Explicitly annotate the type for get_payload to resolve E0282
        let payload: String = msg.get_payload().unwrap_or_default();
        if socket.send(Message::Text(payload)).await.is_err() {
            break;
        }
    }
}