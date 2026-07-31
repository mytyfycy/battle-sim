pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub cookie_secure: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("Env var missing: DATABASE_URL"))?;

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        let cookie_secure = std::env::var("COOKIE_SECURE")
            .map(|v| v != "false")
            .unwrap_or(true);

        Ok(Config {
            database_url,
            port,
            cookie_secure,
        })
    }
}
