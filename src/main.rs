use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};

const PROXY_PORT: i32 = 1243;
const REMOTE_PORT: &str = "127.0.0.1:25565";

fn read_varint(buf: &[u8]) -> Option<(i32, usize)> {
    let mut num = 0;
    let mut shift = 0;

    for (i, byte) in buf.iter().enumerate() {
        // leer últimos 7 bits
        let val = (byte & 0b01111111) as i32;
        num |= val << shift;

        // si el primer bit es 0, es el ultimo byte
        if byte & 0b10000000 == 0 {
            return Some((num, i + 1));
        }

        shift += 7;
        if shift >= 32 {
            return None;
        }
    }

    None
}

fn inspect_packet(buffer: &mut Vec<u8>) {
    loop {
        let (length, len_size) = match read_varint(&buffer) {
            Some(v) => v,
            None => break,
        };

        if buffer.len() < len_size + length as usize {
            break;
        }

        let packet = &buffer[len_size..len_size + length as usize];

        // leer packet id
        if let Some((packet_id, id_size)) = read_varint(packet) {
            println!("Packet ID: {}", packet_id);

            let data = &packet[id_size..];
            println!("Data (hex): {:02X?}", data);
        }

        buffer.drain(0..len_size + length as usize);
    }
}

async fn forward<R, W>(mut from: R, mut to: W)
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut temp = [0u8; 4096];
    let mut buffer = Vec::new();

    loop {
        let n = from.read(&mut temp).await.unwrap();
        if n == 0 {
            break;
        }

        buffer.extend_from_slice(&temp[..n]);

        inspect_packet(&mut buffer);

        if to.write_all(&temp[..n]).await.is_err() {
            break;
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
        forward(c1, s2),
        forward(s1, c2)
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