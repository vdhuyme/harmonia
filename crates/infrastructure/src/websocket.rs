use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
};
use domain::models::QueueItem;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use uuid::Uuid;

/// WebSocket state containing the broadcast channel for this room
pub struct RoomWebSocketState {
    pub tx: broadcast::Sender<RoomEvent>,
}

impl Default for RoomWebSocketState {
    fn default() -> Self {
        Self::new()
    }
}

impl RoomWebSocketState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }
}

/// Events that can be broadcast to WebSocket clients
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum RoomEvent {
    QueueUpdated(Vec<QueueItem>),
    SongStarted(Box<QueueItem>),
    SongEnded,
    UserJoined(String),
    UserLeft(String),
}

/// WebSocket handler for a specific room
pub async fn handle_websocket(
    ws: WebSocketUpgrade,
    State(state): State<Arc<RoomWebSocketState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<RoomWebSocketState>) {
    let (write, mut read) = socket.split();
    let mut rx = state.tx.subscribe();

    // Wrap write in Arc<Mutex> to share between tasks
    let write = Arc::new(Mutex::new(write));
    let write_for_broadcast = write.clone();
    let write_for_ping = write.clone();

    // Spawn a task to forward messages from the broadcast channel to the WebSocket
    let broadcast_task = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            let json = serde_json::to_string(&event).unwrap();
            let mut write = write_for_broadcast.lock().await;
            if write.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // Spawn a task to read messages from the WebSocket and respond to ping
    let ping_task = tokio::spawn(async move {
        while let Some(Ok(Message::Ping(data))) = read.next().await {
            // Respond to ping
            let mut write = write_for_ping.lock().await;
            if write.send(Message::Pong(data)).await.is_err() {
                break;
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = broadcast_task => {},
        _ = ping_task => {},
    }
}

/// Manager for all room WebSocket states
pub struct WebSocketManager {
    rooms: std::collections::HashMap<Uuid, Arc<RoomWebSocketState>>,
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            rooms: std::collections::HashMap::new(),
        }
    }

    pub fn get_or_create_room(
        &mut self,
        room_id: Uuid,
    ) -> Arc<RoomWebSocketState> {
        self.rooms
            .entry(room_id)
            .or_insert_with(|| Arc::new(RoomWebSocketState::new()))
            .clone()
    }

    pub fn broadcast(&self, room_id: Uuid, event: RoomEvent) {
        if let Some(state) = self.rooms.get(&room_id) {
            let _ = state.tx.send(event);
        }
    }
}
