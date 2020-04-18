use hyper::{header::HeaderName, Body, Request};
use std::env;
use std::net::{IpAddr, SocketAddr};
use uuid::Uuid;

pub fn env(name: &str) -> Option<String> {
    env::var(name).ok()
}

pub fn env_crit(name: &str) -> String {
    env(name)
        .filter(|x| !x.trim().is_empty())
        .expect(format!(r#"The critical env variable: "{}" not given"#, name).as_str())
}

pub fn gen_uuid() -> String {
    Uuid::new_v4()
        .to_hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_owned()
}

pub fn or<T: Sized>(cond: bool, truth_val: T, false_val: T) -> T {
    if cond {
        truth_val
    } else {
        false_val
    }
}

pub fn extract_client_ip_from_req(req: &Request<Body>, remote_addr: SocketAddr) -> Option<IpAddr> {
    let header_x_forwarded_for = req
        .headers()
        .get(HeaderName::from_static("x-forwarded-for"))
        .and_then(|v| v.to_str().ok());
    let remote_ip = remote_addr.ip().to_string();

    let ips = header_x_forwarded_for.or_else(|| Some(remote_ip.as_str()));
    ips.and_then(|ips| ips.split(",").nth(0))
        .map(|ip| ip.trim())
        .and_then(|ip| ip.parse::<IpAddr>().ok())
}
