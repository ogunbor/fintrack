use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")?;
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()?;

        Ok(Self {
            database_url,
            host,
            port,
        })
    }
}