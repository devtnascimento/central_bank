use crate::{connection, io};
use protocol::{
    message::{response, Status, User},
    serde_json,
};
use redis::{aio::Connection, AsyncCommands, Client, RedisError};
use std::{borrow::BorrowMut, cell::Cell};

pub struct Redis {
    pub client: Client,
    pub connection: Cell<Connection>,
}

impl Redis {
    pub async fn new() -> io::Result<Redis> {
        let client = Client::open("redis://127.0.0.1/")?;
        let connection = client.get_async_connection().await?;
        Ok(Redis {
            client,
            connection: Cell::new(connection),
        })
    }

    pub async fn register(&mut self, users: Vec<User>) -> io::Result<Status> {
        for user in users {
            let key = user.pix_key.as_str();
            let user_json = serde_json::to_string(&user)?;
            self.connection
                .get_mut()
                .set::<&str, String, String>(key, user_json)
                .await?;
        }
        Ok(Status::Ok)
    }

    pub async fn get(&mut self, key: &String) -> io::Result<response::Pix> {
        println!("redis get key");
        let con = self.connection.get_mut();
        let user_json: Result<Option<String>, RedisError> = con.get(key).await;
        match user_json {
            Ok(Some(json_string)) => {
                let user = serde_json::from_str(&json_string)?;
                return Ok(response::Pix {
                    status: Status::Ok,
                    user: Some(user),
                });
            }
            Ok(None) => {
                eprintln!("key {} not found in redis", key);
                return Ok(response::Pix {
                    status: Status::Error(format!("key {} not found", key)),
                    user: None,
                });
            }
            Err(e) => {
                println!("redis error = {}", e);
                return Err(Box::new(e));
            }
        };
    }
}
