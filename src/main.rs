use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};

mod utils;

const PROXY_PORT: i32 = 1243;
const REMOTE_PORT: &str = "127.0.0.1:25565";

enum Directions {
    ServerToClient,
    ClientToServer
}

enum FilterResult {
    Send(Vec<u8>),
    Cancel,
    Incomplete
}

fn inspect_packet(buffer: &mut Vec<u8>, direction: &Directions) -> FilterResult {
    let (size, len_size) = match utils::read_varint(&buffer) {
        Some(v) => v,
        None => return FilterResult::Incomplete
    };

    let total_size = len_size + size as usize;
    if buffer.len() < total_size {
        return FilterResult::Incomplete
    }

    let packet = buffer.drain(..total_size).collect::<Vec<u8>>();
    FilterResult::Send(packet)
}

async fn forward<R, W>(mut from: R, mut to: W, direction: Directions)
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
            match inspect_packet(&mut buffer, &direction) {
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

    let (c1, c2) = client.into_split();
    let (s1, s2) = server.into_split();

    tokio::join!(
        forward(c1, s2, Directions::ClientToServer),
        forward(s1, c2, Directions::ServerToClient)
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
