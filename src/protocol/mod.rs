mod codec;
mod varint;
mod packet_reader;
mod packets;

use core::fmt;

pub use codec::inspect_packet;
pub use varint::{read_string, read_ushort, read_varint};
pub use packet_reader::PacketReader;
pub use packets::ConnectionState;

use crate::protocol::varint::{write_string, write_ushort, write_varint};

pub enum Direction {
    ServerToClient,
    ClientToServer,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::ClientToServer => "Client -> Server",
            Self::ServerToClient => "Server -> Client"
        })
    }
}

pub enum FilterResult {
    Send(Vec<u8>),
    Cancel,
    Incomplete,
}
