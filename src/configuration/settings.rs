#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl Settings {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")?;

        let jwt_secret = std::env::var("JWT_SECRET")?;
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()?;

        Ok(Self {
            database_url,
            jwt_secret,
            host,
            port,
        })
    }
}
