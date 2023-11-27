use crate::{connection, io};
use protocol::{message, serde_json, User};
use redis::{AsyncCommands, Client, Connection};

#[derive(Debug)]
pub struct Redis {
    pub client: Client,
    pub connection: Connection,
}

impl Redis {
    pub async fn new() -> io::Result<Redis> {
        let client = Client::open("redis://127.0.0.1/")?;
        let mut connection = client.get_async_connection().await?;
        Ok(Redis { client, connection })
    }

    pub async fn register(&self, users: Vec<User>) -> io::Result<Status> {
        for user in users {
            let key = user.pix_key.as_str();
            let user_json = serde_json::to_string(&user)?;
            self.connection.set(key, user_json).await?;
        }
    }
}
