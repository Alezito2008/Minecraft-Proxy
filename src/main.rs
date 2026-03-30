use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};

mod utils;
mod protocol;

use protocol::{Direction, FilterResult, inspect_packet};

use crate::protocol::ConnectionState;

const PROXY_PORT: i32 = 1243;
const REMOTE_PORT: &str = "127.0.0.1:25565";

async fn forward<R, W>(
    mut from: R,
    mut to: W,
    direction: Direction,
    state: Arc<Mutex<ConnectionState>>
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

        loop {
            let result = {
                let mut current_state = state.lock().unwrap();
                inspect_packet(&mut buffer, &direction, &mut current_state)
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

    let state = Arc::new(Mutex::new(ConnectionState::Handshaking));

    let (c_read, c_write) = client.into_split();
    let (s_read, s_write) = server.into_split();

    let state_c2s = Arc::clone(&state);
    let state_s2c = Arc::clone(&state);

    tokio::join!(
        forward(c_read, s_write, Direction::ClientToServer, state_c2s),
        forward(s_read, c_write, Direction::ServerToClient, state_s2c),
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
