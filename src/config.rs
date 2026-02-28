use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub address: String,
    pub port: u16,
    pub db_url: String,
    #[serde(default = "default_log_level")]
    pub rust_log: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let _ = dotenv();
        envy::from_env::<Self>().expect("failed to parse env vars")
    }
}

fn default_log_level() -> String {
    format!(
        "info,{}=debug,sqlx=warn,tower_http=info",
        env!("CARGO_CRATE_NAME")
    )
}
