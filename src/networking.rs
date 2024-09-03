use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::{broadcast, mpsc};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub sender: String,
    pub content: String,
}

const DISCOVERY_PORT: u16 = 8888;
const BROADCAST_ADDR: &str = "255.255.255.255:8888";

pub async fn start_server(addr: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    let (tx, mut rx) = broadcast::channel(10);

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let id = 1; // Assign a unique ID to each connection
            handle_connection(socket, tx, rx).await.unwrap();
        });
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) -> tokio::io::Result<()> {
    let mut buffer = vec![0; 1024];

    loop {
        tokio::select! {
            result = socket.read(&mut buffer) => {
                let n = result?;
                if n == 0 {
                    break;
                }

                let msg = String::from_utf8_lossy(&buffer[..n]);
                let full_msg = format!("{}: {}", 1, msg); // Example ID
                tx.send(full_msg).unwrap();
            }
            result = rx.recv() => {
                let msg = result.unwrap();
                socket.write_all(msg.as_bytes()).await?;
            }
        }
    }

    Ok(())
}

pub async fn broadcast_presence() -> tokio::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;
    
    let message = b"DISCOVER_PEER";
    loop {
        socket.send_to(message, BROADCAST_ADDR).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

pub async fn listen_for_peers() -> tokio::io::Result<()> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", DISCOVERY_PORT)).await?;
    let mut buffer = [0; 1024];
    
    loop {
        let (len, src) = socket.recv_from(&mut buffer).await?;
        let msg = String::from_utf8_lossy(&buffer[..len]);
        if msg == "DISCOVER_PEER" {
            println!("Discovered peer at: {}", src);
        }
    }
}
