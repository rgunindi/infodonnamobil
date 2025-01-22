#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod component;
use component::app::App;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus_cli_config::*;
use http::Method;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};

use std::panic;

pub fn setup_panic_hook() {
    panic::set_hook(Box::new(|info| {
        // Panik detaylarını logla
        eprintln!("Rust Panic: {:?}", info);
    }));
}
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    setup_panic_hook();
    // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
    // and we use the generated address the CLI gives us

    use server_fn::client::set_server_url;

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
        ])
        .allow_origin(Any);
    let mut address = fullstack_address_or_localhost();

    // Set up the axum router
    let router = axum::Router::new().layer(cors).register_server_functions();
    // This will add a fallback route to the router that will serve your component and server functions
    // .serve_dioxus_application(ServeConfigBuilder::default(), App);

    // Try binding to the address, if fails try alternative ports
    let listener = loop {
        match tokio::net::TcpListener::bind(address).await {
            Ok(listener) => {
                println!("Server started successfully on {}", address);
                break listener;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AddrInUse {
                    // Try next port
                    let new_port = address.port() + 1;
                    address.set_port(new_port);
                    println!("Port in use, trying port {}", new_port);
                    continue;
                }
                panic!("Failed to bind to address: {}", e);
            }
        }
    };

    let router = router.into_make_service();
    println!("Local axum server running on {}", address);
    let ip = "https://backoffice.koyeb.app";
    // let ip2 = "http://192.168.1.109:8080";
    set_server_url(ip);
    info!("SERVERIP:{0}", server_fn::client::get_server_url());
    println!("SERVERIP:{0}", server_fn::client::get_server_url());
    axum::serve(listener, router).await.unwrap();
}
#[cfg(not(feature = "server"))]
fn main() {
    setup_panic_hook();
    use dioxus::{
        fullstack::prelude::server_fn::client::{get_server_url, set_server_url},
        logger::tracing::info,
    };
    if get_server_url().is_empty() {
        println!("IP ADRESI BOS!!");
        let ip = "https://backoffice.koyeb.app";
        let _serverurl = format!(
            "{ip}:{}",
            std::env::var("PORT").unwrap_or_else(|_| "8080".to_string())
        )
        .leak();
        set_server_url(ip);
        // set_server_url("http://192.168.1.109:8080");
        info!("SERVERIP:{0}", get_server_url());
        println!("SERVERIP:{0}", get_server_url());
    }
    info!("SERVERIP:{0}", get_server_url());
    println!("SERVERIP:{0}", get_server_url());
    dioxus::launch(App);
}
