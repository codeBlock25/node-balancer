use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Address {
    pub address: [Connection; 3],
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Connection {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub(crate) weight: u8,
    pub(crate) is_active: bool,
    pub(crate) last_response_time: usize,
    pub client: Client
}

impl Default for Server {
    fn default() -> Self {
        Server{host : "0.0.0.0".to_string(), port : 8080, weight : 1,is_active: false, last_response_time: 0, client: Client::default()}
    }
}




// impl Copy for Server {}
