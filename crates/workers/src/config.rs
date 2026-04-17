#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub database_url: String,
    pub redis_url: String,
    pub check_interval_secs: u64,
}

impl WorkerConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env_or(
                "DATABASE_URL",
                "postgres://harmonia:harmonia@postgres:5432/harmonia",
            ),
            redis_url: env_or("REDIS_URL", "redis://redis:6379"),
            check_interval_secs: env_or("WORKER_CHECK_INTERVAL_SECS", "10")
                .parse()
                .unwrap_or(10),
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
