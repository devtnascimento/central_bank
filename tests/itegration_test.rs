use protocol::message::{AsyncReadExt, AsyncWriteExt, Status, TcpStream};
use protocol::{message, serde_json};
use redis::{self, AsyncCommands};
use std::error::Error;
use tokio;

#[tokio::test]
async fn register() -> Result<(), Box<dyn Error>> {
    let full_address = "127.0.0.1:8080";

    let user1 = message::User {
        bank_name: String::from("bank1"),
        account_number: String::from("123"),
        name: String::from("foo"),
        last_name: String::from("bar"),
        cpf: String::from("12312312312"),
        pix_key: String::from("some_cool_key"),
    };

    let user2 = message::User {
        bank_name: String::from("bank1"),
        account_number: String::from("223"),
        name: String::from("Thiago"),
        last_name: String::from("Nascimento"),
        cpf: String::from("12312312311"),
        pix_key: String::from("some_coolest_key"),
    };

    let users = vec![user1, user2];

    let pix = message::register::Pix {
        message_type: message::MessageType::Register,
        users,
    };

    let request = serde_json::to_string(&pix)?;

    let mut stream = TcpStream::connect(full_address).await?;
    stream.write_all(request.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let resp = String::from_utf8_lossy(&buffer).to_string();
    println!("resp = {}", resp);
    let status: Status = serde_json::from_str(&resp)?;

    assert_eq!(Status::Ok, status);

    Ok(())
}

#[tokio::test]
async fn request() -> Result<(), Box<dyn Error>> {
    let full_address = "127.0.0.1:8080";

    let pix = message::request::Pix {
        message_type: message::MessageType::Request,
        key: String::from("some_coolest_key"),
    };

    let request = serde_json::to_string(&pix)?;

    let mut stream = TcpStream::connect(full_address).await?;
    println!("request = {}", request);
    stream.write_all(request.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let resp = String::from_utf8_lossy(&buffer).to_string();
    println!("resp = {}", resp);
    let resp_pix: message::response::Pix = serde_json::from_str(&resp)?;

    let user = message::User {
        bank_name: String::from("bank1"),
        account_number: String::from("223"),
        name: String::from("Thiago"),
        last_name: String::from("Nascimento"),
        cpf: String::from("12312312311"),
        pix_key: String::from("some_coolest_key"),
    };

    let client = redis::Client::open("redis://127.0.0.1/")?;

    let con = &mut client.get_async_connection().await?;

    let result: Option<String> = con.get(pix.key).await?;

    if let Some(value) = result {
        println!("value form redis on test = {}", value);
        let user_: message::User = serde_json::from_str(&value)?;
        let pix = message::response::Pix {
            status: Status::Ok,
            user: Some(user_),
        };
        assert_eq!(pix, resp_pix);
    } else {
        panic!("key dont found on redis from tests either");
    }

    Ok(())
}
