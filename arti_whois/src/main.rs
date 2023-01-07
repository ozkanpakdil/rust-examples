#[allow(dead_code)]
#[allow(unused_imports)]

use std::collections::HashMap;
use arti_client::{BootstrapBehavior, TorClient};
use job_scheduler::{Job, JobScheduler};
use tor_rtcompat::PreferredRuntime;
use warp::{self, path::FullPath, Filter, http};
use std::io::prelude::*;
use std::net::{IpAddr, TcpStream};
use std::io::BufReader;
use std::env;
use std::str::FromStr;
use futures::{TryFutureExt, TryStreamExt};
use lazy_static::lazy_static;
use tor_rtcompat::testing__::TestOutcome;
use warp::hyper::body::HttpBody;

lazy_static! {
     static ref tor_client: TorClient<PreferredRuntime> = TorClient::builder()
        .bootstrap_behavior(BootstrapBehavior::OnDemand)
        .create_unbootstrapped()
    .ok()
    .unwrap();
}

#[tokio::main]
async fn main() {
    /*let mut sched = JobScheduler::new();

    sched.add(Job::new("0 0 * * * *".parse().unwrap(), move || {
        println!("I get executed every 1 hour!");
        {
            match TorClient::builder()
                .bootstrap_behavior(BootstrapBehavior::OnDemand)
                .create_unbootstrapped()
            {
                Ok(client) => tor_client = client,
                Err(e) => eprintln!("Error creating TOR client: {}", e),
            }
        }
    }));*/


    // let test_arti_http_task = test_arti_http(&tor_client);
    // let test_arti_whois_task = test_arti_whois(&tor_client);
    // let _ = tokio::join!(test_arti_whois_task);

    // println!("{:?}",get_whois_data("1.2.3.4"));

    // println!("{}",get_whois_data("82.12.84.124", &tor_client).await.join("\n"));

    let whois = warp::path("whois")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(whois_handler);

    warp::serve(whois).run(([127, 0, 0, 1], 8080)).await;
}

async fn whois_handler(query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    let ip = query.get("ip");

    let whois_result = get_whois_data("82.12.84.124").await; //perform_whois_lookup(ip);

    Ok(whois_result)
}

/*async fn test_arti_http(tor_client: &TorClient<PreferredRuntime>) -> Result<(), Box<dyn std::error::Error>> {
    // Initiate a connection over Tor to example.com, port 80.
    let mut stream = tor_client.connect(("api.ipify.org", 80)).await?;

    use futures::io::{AsyncReadExt, AsyncWriteExt};

    // Write out an HTTP request.
    match stream
        .write_all(b"GET / HTTP/1.1\r\nHost: api.ipify.org\r\nConnection: close\r\n\r\n")
        .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error writing to stream: {}", e);
        }
    };

    // IMPORTANT: Make sure the request was written.
    // Arti buffers data, so flushing the buffer is usually required.
    match stream.flush().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error flushing stream: {}", e);
        }
    };

    // Read and print the result.
    let mut buf = Vec::new();
    match stream.read_to_end(&mut buf).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error reading from stream: {}", e);
        }
    };

    println!("{}", String::from_utf8_lossy(&buf));
    Ok(())
}

async fn test_arti_whois(tor_client: &TorClient<PreferredRuntime>) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = tor_client.connect(("whois.iana.org", 43)).await?;

    use futures::io::{AsyncReadExt, AsyncWriteExt};

    stream.write_all(b"1.2.3.4\r\n").await?;

    stream.flush().await?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;

    println!("{}", String::from_utf8_lossy(&buf));
    Ok(())
}
*/

async fn connect_and_read(domain_whois: &str, data_write: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut stream = tor_client.connect(domain_whois).await?;
    use futures::io::{AsyncReadExt, AsyncWriteExt};
    stream.write_all(format!("{}\n", data_write).as_bytes()).await?; //Send the tld
    stream.flush().await.ok();
    let mut reader = Vec::new();
    stream.read_to_end(&mut reader).await.unwrap();
    Ok(reader)
}

async fn get_tld_server(tld: &str) -> Option<String> {
    for line in connect_and_read( "whois.iana.org:43", tld)
        .await
        .ok()
        .unwrap()
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

    for line in connect_and_read( format!("{}:43", &tld_server[..]).as_str(), domain.as_str())
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
            for line in connect_and_read( format!("{}:43", &server[..]).as_str(), domain.as_str())
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
