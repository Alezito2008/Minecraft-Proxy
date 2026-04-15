use crate::impl_packet_handler;
use crate::protocol::listener::{PacketAction, PacketListener};
use crate::protocol::{PacketReader, Session};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status
// https://minecraft.wiki/w/Java_Edition_protocol/Server_List_Ping

impl_packet_handler!(StatusHandler, packet, session,
    c2s => {
        StatusRequest => on_status_request
        PingPacket => on_ping_packet_request
    }
    s2c => {
        StatusResponse => on_status_response
        PingPacket => on_ping_packet_response
    }
);

pub mod packets {
    use super::*;
    use crate::protocol::varint::*;

    pub struct StatusRequest;

    impl MinecraftPacket for StatusRequest {
        const ID: i32 = 0x00;
    }

    pub struct StatusResponse {
        pub json_response: String
    }

    impl MinecraftPacket for StatusResponse {
        const ID: i32 = 0x00;

        fn decode(reader: &mut crate::protocol::PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                json_response: reader.read_string()?
            })
        }
    }

    pub struct PingPacket {
        pub payload: i64,
    }

    impl MinecraftPacket for PingPacket {
        const ID: i32 = 0x01;

        fn decode(reader: &mut crate::protocol::PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                payload: reader.read_long()?
            })
        }

        fn encode(&self, buf: &mut Vec<u8>) {
            write_varlong(self.payload, buf);
        }
    }
}