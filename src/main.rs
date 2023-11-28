mod connection;
mod database;
mod io;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server listening at 127.0.0.1:8080");

    while let Ok((socket, addr)) = listener.accept().await {
        tokio::spawn(connection::handle(socket, addr));
    }

    Ok(())
}
