use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};

mod utils;
mod protocol;

use protocol::{Direction, FilterResult, inspect_packet};

use crate::protocol::Session;

const PROXY_PORT: i32 = 1243;
const REMOTE_PORT: &str = "127.0.0.1:25565";

async fn forward<R, W>(
    mut from: R,
    mut to: W,
    direction: Direction,
    session: Arc<Mutex<Session>>
)
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut temp = [0u8; 4096];
    let mut buffer = Vec::new();

    loop {
        let n = match from.read(&mut temp).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break
        };
        buffer.extend_from_slice(&temp[..n]);

        // Loop por si se envía más de un packet a la vez
        loop {
            let result = {
                let mut current_session = session.lock().unwrap();
                inspect_packet(&mut buffer, &direction, &mut current_session)
            };

            match result {
                FilterResult::Send(packet) => {
                    if to.write_all(&packet).await.is_err() {
                        return;
                    }
                },
                FilterResult::Cancel => continue,
                FilterResult::Incomplete => break
            }
        }
    }
}

async fn handle_connection(client: TcpStream) {
    let server = match TcpStream::connect(REMOTE_PORT).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Couldn't connect to server: {e}");
            return
        }
    };

    let session = Arc::new(Mutex::new(
        Session {
            state: protocol::ConnectionState::Handshaking,
            compression_threshold: -1
        }
    ));

    let (c_read, c_write) = client.into_split();
    let (s_read, s_write) = server.into_split();

    let session_c2s = Arc::clone(&session);
    let session_s2c = Arc::clone(&session);

    tokio::join!(
        forward(c_read, s_write, Direction::ClientToServer, session_c2s),
        forward(s_read, c_write, Direction::ServerToClient, session_s2c),
    );
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{PROXY_PORT}");
    let listener = TcpListener::bind(&addr).await?;
    println!("Proxy listening on {addr}");

    loop {
        let (client, addr) = listener.accept().await?;
        println!("New Connection from: {addr:?}");

        tokio::spawn(async move {
            handle_connection(client).await
        });
    }
}
