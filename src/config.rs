use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub server_config: ServerConfig,
}

impl AppConfig {
    pub(crate) fn from_env() -> AppConfig {
        let host: String = std::env::var("SERVER.HOST").unwrap_or("127.0.0.1".to_string());
        let port: u16 = std::env::var("SERVER.PORT")
            .unwrap_or("6000".to_string())
            .parse()
            .unwrap();
        AppConfig {
            server_config: ServerConfig { host, port },
        }
    }
}
