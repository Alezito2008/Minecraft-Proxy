use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};

pub mod protocol;

use protocol::{Direction, FilterResult, inspect_packet};
use crate::protocol::listener::{PacketListener};

use crate::protocol::Session;

pub struct MinecraftProxy<L: PacketListener> {
    pub proxy_port: u16,
    pub remote_addr: String,
    pub listener: Arc<Mutex<L>>
}

impl<L: PacketListener + Send + 'static> MinecraftProxy<L> {
    pub fn new(proxy_port: u16, remote_addr: &str, packet_listener: L) -> Self {
        Self {
            proxy_port: proxy_port,
            remote_addr: remote_addr.to_string(),
            listener: Arc::new(Mutex::new(packet_listener))
        }
    }

    pub async fn listen(&self) -> std::io::Result<()> {
        let addr = format!("0.0.0.0:{}", self.proxy_port);
        let tcp_listener = TcpListener::bind(&addr).await?;
        println!("Proxy listening on {addr}");

        loop {
            let (client, addr) = tcp_listener.accept().await?;
            let remote_addr = self.remote_addr.clone();
            let packet_listener = Arc::clone(&self.listener);
            println!("New Connection from: {addr:?}");

            tokio::spawn(async move {
                Self::handle_connection(client, remote_addr, packet_listener).await
            });
        }
    }

    async fn handle_connection(
        client: TcpStream,
        remote_addr: String,
        packet_listener: Arc<Mutex<L>>
    ) {
        let server = match TcpStream::connect(remote_addr).await {
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
        let listener_c2s = Arc::clone(&packet_listener);
        let listener_s2c = Arc::clone(&packet_listener);

        tokio::join!(
            Self::forward(c_read, s_write, Direction::ClientToServer, session_c2s, listener_c2s),
            Self::forward(s_read, c_write, Direction::ServerToClient, session_s2c, listener_s2c),
        );
    }

    async fn forward<R, W>(
        mut from: R,
        mut to: W,
        direction: Direction,
        session: Arc<Mutex<Session>>,
        packet_listener: Arc<Mutex<L>>
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
                    let mut listener = packet_listener.lock().unwrap();
                    inspect_packet(&mut buffer, &direction, &mut current_session, &mut *listener)
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
}

