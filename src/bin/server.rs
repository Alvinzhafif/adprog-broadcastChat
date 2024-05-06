use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut write, mut read) = ws_stream.split();
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Receive messages from the client and broadcast them
            msg = read.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if msg.is_text() {
                            if let Some(text) = msg.as_text() {
                                if let Err(e) = bcast_tx.send(text.to_string()) {
                                    eprintln!("Error broadcasting message: {}", e);
                                    break;
                                }
                            }
                        } else if msg.is_close() {
                            println!("Client {addr:?} disconnected");
                            break;
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving message from {addr:?}: {e}");
                        break;
                    }
                    None => {
                        eprintln!("Client {addr:?} disconnected");
                        break;
                    }
                }
            }

            // Send messages received from the broadcast channel to the client
            msg = bcast_rx.recv() => {
                match msg {
                    Ok(msg) => {
                        if let Err(e) = write.send(Message::text(msg)).await {
                            eprintln!("Error sending message to {addr:?}: {}", e);
                            break;
                        }
                    }
                    Err(_) => {
                        eprintln!("Broadcast channel closed");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}