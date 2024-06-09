use std::env;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// 숫자면 +1
// 글자면 그대로
#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap();

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on {}", addr);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Connected by {}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return, // closed
                    Ok(n) => {
                        match std::str::from_utf8(&buf[..n]) {
                            Ok(received_str) => {
                                if let Ok(num) = received_str.trim().parse::<i32>() {
                                    let response = (num + 1).to_string();
                                    socket.write_all(response.as_bytes()).await.unwrap();
                                } else {
                                    socket.write_all(received_str.as_bytes()).await.unwrap();
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to convert buffer to UTF-8: {:?}", e);
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
