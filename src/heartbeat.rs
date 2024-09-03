use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Ensure both of these imports are here

pub async fn send_heartbeat(addr: &str) -> tokio::io::Result<()> {
    let mut stream = TcpStream::connect(addr).await?;
    let heartbeat = b"HEARTBEAT";

    loop {
        stream.write_all(heartbeat).await?; // Send the heartbeat message
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

pub async fn handle_heartbeat(mut socket: TcpStream) -> tokio::io::Result<()> {
    let mut buffer = [0; 9];

    loop {
        let n = socket.read(&mut buffer).await?; // Read the heartbeat message
        if n == 0 || &buffer[..n] != b"HEARTBEAT" {
            println!("Connection lost");
            break;
        }
    }

    Ok(())
}
