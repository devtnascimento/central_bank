mod handler;

use crate::io;
use handler::*;
use protocol::serde_json;
use std::{collections::HashMap, net::SocketAddr};
use tokio::{io::AsyncReadExt, net::TcpStream};

pub async fn handle(mut socket: TcpStream, addr: SocketAddr) -> io::Result<()> {
    println!("Accepted connection from: {}", addr);

    let mut buffer = [0; 1024];
    loop {
        match socket.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                let resp = serde_json::from_str::<HashMap<String, serde_json::Value>>(&msg)?;

                if let Some(msg_type) = resp.get("message_type") {
                    match msg_type.as_str() {
                        Some("Register") => {
                            println!("Received Register request");
                            handle_register(&mut socket, msg.to_string()).await?;
                            break;
                        }
                        Some("Request") => {
                            if let Some(key) = resp.get("key") {
                                let key = key.clone().to_string().trim_matches('"').to_string();
                                handle_request(&mut socket, key).await?;
                                break;
                            } else {
                                handle_error(&mut socket, "pix_key field not found").await?;
                                break;
                            }
                        }
                        Some(_) => {
                            handle_error(&mut socket, "Invalid message_type field").await?;
                            break;
                        }
                        None => {
                            handle_error(&mut socket, "Missing message_type field").await?;
                            break;
                        }
                    }
                }
            }
            Ok(_) => {
                println!("connection closed by {}: {}", addr, addr);
                break;
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}
