mod handler;

use crate::io;
use handler::*;
use protocol::{message, serde_json};
use std::{collections::HashMap, net::SocketAddr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn handle(mut socket: TcpStream, addr: SocketAddr) -> io::Result<()> {
    println!("Accepted connection from: {}", addr);

    let mut buffer = [0; 1024];
    loop {
        match socket.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                let resp = serde_json::from_str::<HashMap<String, serde_json::Value>>(&msg)?;

                println!("msg = {}", msg);

                if let Some(msg_type) = resp.get("message_type") {
                    match msg_type.as_str() {
                        Some("Register") => {
                            handle_register(&msg.to_string()).await?;
                        }
                        Some("Request") => {
                            if let Some(key) = resp.get("pix_key") {
                                handle_request(&msg.to_string()).await?;
                            } else {
                                unreachable!("pix_key not found");
                            }
                        }
                        Some(_) => {
                            return Err(Box::new(io::RequestError::Other("Invalid message_type")));
                        }
                        None => {
                            return Err(Box::new(io::RequestError::Other(
                                "Missing message_type field",
                            )));
                        }
                    }
                }
            }
            Ok(_) => {
                println!("connection closed by {}: {}", addr, addr);
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
            }
        }
    }
}
