use std::{env, io::Error};

use futures_util::{SinkExt, StreamExt};
use regex::Regex;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();
    while let Some(Ok(msg)) = read.next().await {
        if msg.is_text() {
            let text = msg.to_text().unwrap();
            let re = Regex::new(r"(\d+)").unwrap();
            if let Some(captures) = re.captures(text) {
                if let Ok(n) = captures[1].parse::<usize>() {
                    for _i in 0..n {
                        write.send(Message::text(text)).await.unwrap();
                    }
                }
            } else {
                println!("{text}");
                write.send(Message::Text("Invalid message:".to_string())).await.unwrap();
            }
        }
    }
}