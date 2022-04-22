extern crate env_logger;
extern crate serde;

mod models;
mod config;
mod out_going;
mod in_coming;
mod connection;
mod util;


use std::{future, io, time::Duration};
use serde_json;
use std::fs::OpenOptions;
use std::io::Read;
use reqwest::{header, Client, ClientBuilder, Url};
use actix_web::{App, HttpServer, Responder, services, web};
use actix_web::middleware::Logger;
use awc::http::StatusCode;
use crate::config::AppConfig;
use crate::models::{Address, Connection, Server};
use dotenv::dotenv;
use crate::connection::get_connections;
use crate::in_coming::get_in;
use crate::out_going::get_out;
use crate::util::{InClient, OutUrl};


const ADDRESS_FILE: &str = "address.json";

fn request_handler (client: Client, servers: &Vec<Sever>, server_index: &usize) -> impl Responder{
    let server: Server = servers.clone().get(server_index).unwrap().into();
    let url = format!("http://{}:{}", server.host, server.port);
    client.get(url)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=println!()");
    env_logger::init();

    let config = AppConfig::from_env();


    println!("Fetching Connection");
    let connections = get_connections();
    println!("{len} Connections found", len = connections.len());

    let mut client = Client::default();


    let mut servers = Vec::new();
    for connection in connections {
        println!("Loading server service config");

        let resp = client.get(&format!("http://{}:{}/greeting", connection.host, connection.port))
            .insert_header(("User-Agent", "actix-web/3.0"))
            .send()     // <- Send request
            .await;
        match resp {
            Ok(response) => {
                println!("{:?}", response);
                if response.status() == StatusCode::OK {
                    servers.push(Server {
                        host: connection.host,
                        port: connection.port,
                        weight: 1,
                        is_active: response.status() == StatusCode::OK,
                        last_response_time: 0,
                        client: ClientBuilder::new()
                        .http2_adaptive_window(true)
                        .tcp_keepalive(Duration::new(150, 0))
                        .tcp_nodelay(true) // disable Nagle
                        // .connect_timeout(Duration::new(150, 0))
                        .connection_verbose(true)
                        .build()
                        .expect("Failed creating out client pool")
                    })
                }
            }
            Err(err) => {
                println!("Could not make a connect to http://{}{}", connection.host, connection.port)
            }
        };
    }

    println!(
        "Load Balance up and running on http://{}:{}",
        config.server_config.host, config.server_config.port
    );

    let redirect_url = OutUrl {
        url: Url::parse(&format!("http://localhost:{}", CLIENTPORT)).unwrap(),
        out_client,
    };
    let in_c = InClient { in_client };
    println!("Redirect URL: {}", redirect_url.url);

    let s_out = HttpServer::new(move || {
        App::new()
            .default_service(web::route().to(get_out))
            .data(redirect_url.clone())
    })
        .bind((&config.server_config.host, &config.server_config.port))?
        .run();

    let s_in = HttpServer::new(move || {
        App::new()
            .default_service(web::route().to(get_in))
            .data(in_c.clone())
    })
        .bind((&config.server_config.host, &config.server_config.port))?
        .run();

    future::try_join(s_in, s_out).await?;
    Ok(())
}
