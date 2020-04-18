extern crate rust_hyper_server_template;
use colored::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rust_hyper_server_template::constants;
use rust_hyper_server_template::utils;
use std::net::IpAddr;
use std::{convert::Infallible, net::SocketAddr};

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World!".into()))
}

#[tokio::main]
async fn main() {
    startup();

    let req_service = make_service_fn(move |conn: &hyper::server::conn::AddrStream| {
        let remote_addr = conn.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| async move {
                if let Some(client_ip) = utils::extract_client_ip_from_req(&req, remote_addr) {
                    rust_hyper_server_template::info!("{} {} {}", client_ip, req.method(), req.uri());
                } else {
                    rust_hyper_server_template::info!("{} {}", req.method(), req.uri());
                }
                handle(req).await
            }))
        }
    });

    let addr = SocketAddr::new(
        utils::env(constants::env::HOST)
            .and_then(|host| host.parse::<IpAddr>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_IP),
        utils::env(constants::env::PORT)
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_PORT),
    );
    let server = Server::bind(&addr)
        .http1_keepalive(true)
        .http1_half_close(true)
        .http1_only(false)
        .http2_only(false)
        .http1_writev(true)
        .tcp_sleep_on_accept_errors(true)
        .serve(req_service);

    rust_hyper_server_template::info!("App is serving on: {}", server.local_addr());
    if let Err(e) = server.await {
        rust_hyper_server_template::error!("Server Error: {}", e);
    }
}

fn startup() {
    dotenv::dotenv().ok();
    rust_hyper_server_template::logger::init_logger();
    log_app_env();
}

fn log_app_env() {
    println!("Environment Variables:");
    get_required_env_names()
        .map(|var| (var, utils::env(var).unwrap_or("<NOT_FOUND>".to_owned())))
        .for_each(|(var, val)| {
            println!("  {}: {}", var.color(Color::BrightBlack), val.color(Color::Green));
        });
    println!();
}

fn get_required_env_names() -> impl Iterator<Item = &'static str> {
    include_str!("../.env.example")
        .lines()
        .filter(|line| !line.starts_with("#") && !line.is_empty())
        .map(|line| line.split("=").take(1))
        .flatten()
}
