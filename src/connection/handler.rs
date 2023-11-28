use crate::{database::connection::Redis, io::Result};
use protocol::message::{register, Status};
use protocol::serde_json;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn handle_register(socket: &mut TcpStream, msg: String) -> Result<()> {
    println!("handle_register call");
    let mut redis = Redis::new().await?;
    let pix: register::Pix = serde_json::from_str(&msg)?;
    println!("msg = {}", msg);
    let status = redis.register(pix.users).await?;
    let resp = serde_json::to_string(&status)?;
    println!("resp = {}", resp);
    socket.write_all(&resp.as_bytes()).await?;
    Ok(())
}

pub async fn handle_request(socket: &mut TcpStream, key: String) -> Result<()> {
    println!("handle_request call");
    let mut redis = Redis::new().await?;
    let pix_resp = redis.get(&key).await?;
    let resp = serde_json::to_string(&pix_resp)?;
    println!("resp from cbank = {}", resp);
    socket.write_all(&resp.as_bytes()).await?;
    Ok(())
}

pub async fn handle_error(socket: &mut TcpStream, msg: &str) -> Result<()> {
    let status = Status::Error(msg.to_string());
    let resp = serde_json::to_string(&status)?;
    socket.write_all(&resp.as_bytes()).await?;
    Ok(())
}
