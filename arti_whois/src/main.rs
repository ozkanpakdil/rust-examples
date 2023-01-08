use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{IpAddr};
use std::str::FromStr;

use arti_client::{BootstrapBehavior, TorClient};
use lazy_static::lazy_static;
use tor_rtcompat::PreferredRuntime;
use warp::{self, Filter};
use warp::http::StatusCode;

lazy_static! {
     static ref TOR_CLIENT: TorClient<PreferredRuntime> = create_new_tor_connection();
}

static mut COUNTER: i32 = 0;

fn create_new_tor_connection() -> TorClient<PreferredRuntime> {
    TorClient::builder()
        .bootstrap_behavior(BootstrapBehavior::OnDemand)
        .create_unbootstrapped()
        .ok()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let whois = warp::path("whois")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(whois_handler);

    warp::serve(whois).run(([127, 0, 0, 1], 8016)).await;
}

async fn whois_handler(query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    let default = "".to_string();
    let ip = query.get("ip").unwrap_or(&default);
    if ip.is_empty() {
        return Ok(warp::reply::with_status("empty ip".to_string(), StatusCode::BAD_REQUEST));
    }
    unsafe {
        COUNTER += 1;
        println!("{}-ip:{}", COUNTER, ip);
    }
    Ok(match IpAddr::from_str(ip) {
        Ok(ip_addr) => warp::reply::with_status(get_whois_data(ip).await, StatusCode::OK),
        Err(_) => warp::reply::with_status("wrong ip".to_string(), StatusCode::BAD_REQUEST),
    })
}

async fn whois_handler_old(query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    let default = "".to_string();
    let ip = query.get("ip").unwrap_or(&default);
    unsafe {
        COUNTER += 1;
        println!("{}-ip:{}", COUNTER, ip);
    }
    if IpAddr::from_str(ip).is_ok() {
        let response=get_whois_data(ip).await;
        // TODO maybe in the future we can restart from code.
        // if response.contains("access denied"){
        //
        // }
        return Ok(warp::reply::with_status(response, StatusCode::OK));
    }
    Ok(warp::reply::with_status("wrong ip".to_string(), StatusCode::BAD_REQUEST))
}

async fn connect_and_read(domain_whois: &str, data_write: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut stream = TOR_CLIENT.connect(domain_whois).await?;
    use futures::io::{AsyncReadExt, AsyncWriteExt};
    stream.write_all(format!("{}\n", data_write).as_bytes()).await?; //Send the tld
    stream.flush().await.ok();
    let mut reader = Vec::new();
    stream.read_to_end(&mut reader).await.unwrap();
    Ok(reader)
}

async fn get_tld_server(tld: &str) -> Option<String> {
    for line in connect_and_read("whois.iana.org:43", tld)
        .await
        .ok()
        .unwrap_or(Vec::new())
        .lines() {
        let line = line.unwrap();
        let parts: Vec<String> = line.splitn(2, ":").map(|x| x.to_string()).collect();
        if parts.len() == 2 {
            if parts[0].to_lowercase() == "whois" {
                return Some(parts[1].trim().to_string());
            }
        }
    }

    return None;
}

async fn get_whois_data(domain: &str) -> String {
    let mut next_server: Option<String> = None;
    let mut whois_data = Vec::new();

    let domain = domain.to_string();
    let tld = domain.split(".").last().unwrap(); // Get the top-level domain
    let tld_server = match get_tld_server(tld).await {
        Some(server) => server,
        None => {
            whois_data.push(format!("Can't find a whois server for {}", domain));
            whois_data.join("\n")
        }
    };

    for line in connect_and_read(format!("{}:43", &tld_server[..]).as_str(), domain.as_str())
        .await
        .ok()
        .unwrap()
        .lines() {
        let line = line.unwrap();
        let parts: Vec<String> = line.splitn(2, ":").map(|x| x.to_string()).collect();
        if parts.len() == 2 {
            if parts[0].to_lowercase().trim() == "whois server" && !parts[1].trim().is_empty() {
                next_server = Some(parts[1].trim().to_owned());
            }
        }
        whois_data.push(line);
    }

    match next_server {
        Some(server) => {
            for line in connect_and_read(format!("{}:43", &server[..]).as_str(), domain.as_str())
                .await
                .ok()
                .unwrap()
                .lines() {
                let line = line.unwrap();
                whois_data.push(line);
            }
        }
        None => {}
    }

    whois_data.join("\n")
}
