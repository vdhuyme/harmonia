use crate::{AppError, AppState};
use axum::{extract::State, http::StatusCode, Json};
use domain::{ConnectProviderRequest, MapRoomRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProviderAccountResponse {
    pub provider_account_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoomMappingResponse {
    pub mapping_id: String,
}

pub async fn connect_provider(
    State(_state): State<AppState>,
    Json(_request): Json<ConnectProviderRequest>,
) -> Result<(StatusCode, Json<ProviderAccountResponse>), AppError> {
    // TODO: Implement in Phase 4+
    // - Validate provider
    // - Encrypt token
    // - Store in database
    Err(AppError::InternalError("Not implemented yet".to_string()))
}

pub async fn map_room(
    State(_state): State<AppState>,
    Json(_request): Json<MapRoomRequest>,
) -> Result<(StatusCode, Json<RoomMappingResponse>), AppError> {
    // TODO: Implement in Phase 3+
    // - Validate room and provider account exist
    // - Create mapping
    Err(AppError::InternalError("Not implemented yet".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_admin_handlers_exist() {
        // Handlers will be fully tested in integration tests once DB is connected
    }
}
