#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod component;
use component::app::App;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus_cli_config::*;
use http::Method;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    // Get the address the server should run on. If the CLI is running, the CLI proxies fullstack into the main address
    // and we use the generated address the CLI gives us

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
    let address = fullstack_address_or_localhost();

    // Set up the axum router
    let router = axum::Router::new().layer(cors).register_server_functions();
    // This will add a fallback route to the router that will serve your component and server functions
    // .serve_dioxus_application(ServeConfigBuilder::default(), App);

    println!("Local axum server:{address}");
    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
#[cfg(not(feature = "server"))]
fn main() {
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
        info!("SERVERIP:{0}", get_server_url());
        println!("SERVERIP:{0}", get_server_url());
    }
    dioxus::launch(App);
    // infodonnamobil::main();
}
