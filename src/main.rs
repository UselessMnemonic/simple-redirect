use std::env::args;
use std::process::exit;
use async_std::net::TcpListener;
use futures::{AsyncWriteExt, StreamExt};

#[async_std::main]
async fn main() {
    let args: Vec<String> = args().collect();
    let target = args.get(1).expect("Target URL must be specified");
    let redirect_response = format!("HTTP/1.1 301 Moved Permanently\r\nLocation: {target}\r\nConnection: close\r\n\r\n");
    let redirect_response_bytes = redirect_response.as_bytes();

    let listener = TcpListener::bind("0.0.0.0:80").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |client| async {
            match client {
                Ok(mut stream) => {
                    let _ = stream.write_all(redirect_response_bytes).await;
                    let _ = stream.flush().await;
                    let _ = stream.close().await;
                },
                _ => {}
            }
        })
        .await;
}
