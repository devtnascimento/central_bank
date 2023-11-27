use crate::{database::connection::Redis, io::Result};
use protocol::message::{register, request};

pub async fn handle_register(msg: String) -> Result<()> {
    let redis = Redis::new().await?;
    redis.register(msg.users).await?
}

pub async fn handle_request(msg: &String) -> Result<()> {
    todo!()
}
