#[derive(Clone, Debug)]
pub struct Config {
    pub api_host: String,
    pub api_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub spotify_client_id: String,
    pub spotify_redirect_uri: String,
    pub youtube_client_id: String,
    pub youtube_redirect_uri: String,
    pub youtube_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            api_host: env_or("API_HOST", "0.0.0.0"),
            api_port: env_or("API_PORT", "3000").parse().unwrap_or(3000),
            database_url: env_or(
                "DATABASE_URL",
                "postgres://harmonia:harmonia@postgres:5432/harmonia",
            ),
            redis_url: env_or("REDIS_URL", "redis://redis:6379"),
            jwt_secret: env_or("JWT_SECRET", "dev-jwt-secret"),
            spotify_client_id: env_or("SPOTIFY_CLIENT_ID", "spotify-client-id"),
            spotify_redirect_uri: env_or(
                "SPOTIFY_REDIRECT_URI",
                "http://localhost:3000/auth/spotify/callback",
            ),
            youtube_client_id: env_or("YOUTUBE_CLIENT_ID", "youtube-client-id"),
            youtube_redirect_uri: env_or(
                "YOUTUBE_REDIRECT_URI",
                "http://localhost:3000/auth/youtube/callback",
            ),
            youtube_api_key: env_or("YOUTUBE_API_KEY", "youtube-api-key"),
        }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
