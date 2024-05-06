use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            // Task for reading user messages from standard input and sending them to the server
            line = stdin.next_line() => {
                match line {
                    Ok(Some(input)) => {
                        if let Err(e) = ws_stream.send(Message::text(input)).await {
                            eprintln!("Error sending message to server: {}", e);
                            break;
                        }
                    }
                    Ok(None) => {
                        // End of input, terminate the loop
                        break;
                    }
                    Err(e) => {
                        eprintln!("Error reading from stdin: {}", e);
                        break;
                    }
                }
            }

            // Task for receiving messages from the server and displaying them for the user
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        // Display the message received from the server
                        if let Some(text) = msg.as_text() {
                            println!("Server: {}", text);
                        }
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving message from server: {}", e);
                        break;
                    }
                    None => {
                        // Server closed the connection, terminate the loop
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}