use arti_client::{BootstrapBehavior, TorClient};
use futures::FutureExt;

#[tokio::main]
async fn main() {
    test_arti().await;
    println!("Hello, world!");
}

async fn test_arti() -> Result<(), Box<dyn std::error::Error>> {
    let tor_client = TorClient::builder()
        .bootstrap_behavior(BootstrapBehavior::OnDemand)
        .create_unbootstrapped();
    // Initiate a connection over Tor to example.com, port 80.
    let mut stream = tor_client.unwrap().connect(("api.ipify.org", 80)).await?;


    println!("1");
    
    use futures::io::{AsyncReadExt, AsyncWriteExt};
    
    // Write out an HTTP request.
    stream
    .write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n")
    .await?;
    println!("2");
    
    // IMPORTANT: Make sure the request was written.
    // Arti buffers data, so flushing the buffer is usually required.
    stream.flush().await;
    println!("3");
    
    // Read and print the result.
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;
    println!("4");

    println!("1- {}", String::from_utf8_lossy(&buf));
    Ok(())
}
