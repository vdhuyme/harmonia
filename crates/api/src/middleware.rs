use crate::error::AppError;
use crate::jwt::{Claims, JwtHandler};
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use std::sync::Arc;

/// Authenticated user extracted from JWT token
#[derive(Debug, Clone, PartialEq)]
pub struct AuthUser {
    /// User ID
    pub user_id: String,
    /// User role
    pub role: String,
    /// Optional room ID for scoped access
    pub room_id: Option<String>,
}

impl AuthUser {
    /// Check if user is admin
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    /// Check if user can access a specific room
    pub fn can_access_room(&self, room_id: &str) -> bool {
        // Admin can access any room
        if self.is_admin() {
            return true;
        }
        // User can access room if it's in their token
        if let Some(ref token_room) = self.room_id {
            return token_room == room_id;
        }
        false
    }
}

/// JWT extractor for Axum
#[async_trait]
impl FromRequestParts<Arc<JwtHandler>> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        jwt_handler: &Arc<JwtHandler>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::MissingAuthToken)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::InvalidAuthToken);
        }

        let token = &auth_header[7..];
        let claims = jwt_handler.verify_token(token)?;

        if claims.is_expired() {
            return Err(AppError::TokenExpired);
        }

        Ok(AuthUser {
            user_id: claims.sub,
            role: claims.role,
            room_id: claims.room_id,
        })
    }
}

/// Middleware state containing JWT handler
pub type JwtState = Arc<JwtHandler>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_user_is_admin() {
        let user = AuthUser {
            user_id: "user1".to_string(),
            role: "admin".to_string(),
            room_id: None,
        };
        assert!(user.is_admin());

        let non_admin = AuthUser {
            user_id: "user2".to_string(),
            role: "user".to_string(),
            room_id: None,
        };
        assert!(!non_admin.is_admin());
    }

    #[test]
    fn test_auth_user_can_access_room() {
        let admin = AuthUser {
            user_id: "user1".to_string(),
            role: "admin".to_string(),
            room_id: None,
        };
        assert!(admin.can_access_room("any_room"));

        let scoped_user = AuthUser {
            user_id: "user2".to_string(),
            role: "user".to_string(),
            room_id: Some("room1".to_string()),
        };
        assert!(scoped_user.can_access_room("room1"));
        assert!(!scoped_user.can_access_room("room2"));

        let unscoped_user = AuthUser {
            user_id: "user3".to_string(),
            role: "user".to_string(),
            room_id: None,
        };
        assert!(!unscoped_user.can_access_room("any_room"));
    }
}
