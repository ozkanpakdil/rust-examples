use arti_client::{BootstrapBehavior, TorClient};
use job_scheduler::{Job, JobScheduler};
use tor_rtcompat::PreferredRuntime;

#[tokio::main]
async fn main() {
    let mut tor_client = TorClient::builder()
        .bootstrap_behavior(BootstrapBehavior::OnDemand)
        .create_unbootstrapped()
        .unwrap();

    let mut sched = JobScheduler::new();

    sched.add(Job::new("0 0 * * * *".parse().unwrap(), || {
        println!("I get executed every 1 hour!");
        /*{
            tor_client = TorClient::builder()
                .bootstrap_behavior(BootstrapBehavior::OnDemand)
                .create_unbootstrapped()
                .unwrap();
        }*/
    }));


    test_arti_http(&tor_client)
        .await
        .ok();
    test_arti_whois(&tor_client)
        .await
        .ok();
}

async fn test_arti_http(tor_client: &TorClient<PreferredRuntime>) -> Result<(), Box<dyn std::error::Error>> {
    // Initiate a connection over Tor to example.com, port 80.
    let mut stream = tor_client.connect(("api.ipify.org", 80)).await?;

    use futures::io::{AsyncReadExt, AsyncWriteExt};

    // Write out an HTTP request.
    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: api.ipify.org\r\nConnection: close\r\n\r\n")
        .await?;

    // IMPORTANT: Make sure the request was written.
    // Arti buffers data, so flushing the buffer is usually required.
    stream.flush().await?;

    // Read and print the result.
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;

    println!("{}", String::from_utf8_lossy(&buf));
    Ok(())
}

async fn test_arti_whois(tor_client: &TorClient<PreferredRuntime>) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = tor_client.connect(("whois.apnic.net", 43)).await?;

    use futures::io::{AsyncReadExt, AsyncWriteExt};

    stream.write_all(b"1.2.3.4\r\n").await?;

    stream.flush().await.ok();

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;

    println!("{}", String::from_utf8_lossy(&buf));
    Ok(())
}
