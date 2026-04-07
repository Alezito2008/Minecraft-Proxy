use crate::protocol::listener::{PacketAction, PacketListener};
use crate::protocol::{PacketReader, Session};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status
// https://minecraft.wiki/w/Java_Edition_protocol/Server_List_Ping
pub struct StatusHandler;
impl PacketHandler for StatusHandler {
    fn handle_c2s<L: crate::protocol::listener::PacketListener>(
            reader: &mut PacketReader,
            id: i32,
            _session: &mut Session,
            listener: &mut L
        ) -> PacketAction {
        match id {
            StatusRequest::ID => {
                println!("Server Status Requested");
                return listener.on_status_request(&mut StatusRequest);
            }
            PingPacket::ID => {
                if let Some(mut ping_request) = PingPacket::decode(reader) {
                    println!("Sent ping request with payload: {}", ping_request.payload);
                    return listener.on_ping_packet_request(&mut ping_request);
                }
            }
            _ => {}
        }

        PacketAction::Allow
    }


    fn handle_s2c<L: PacketListener>(
            reader: &mut PacketReader,
            id: i32,
            _session: &mut Session,
            listener: &mut L
        ) -> PacketAction {
        match id {
            StatusResponse::ID => {
                if let Some(mut status_response) = StatusResponse::decode(reader) {
                    println!("Status Response: {}", status_response.json_response);
                    return listener.on_status_response(&mut status_response)
                }
            }
            PingPacket::ID => {
                if let Some(mut pong_response) = PingPacket::decode(reader) {
                    println!("Received pong response with payload: {}", pong_response.payload);
                    return listener.on_ping_packet_response(&mut pong_response);
                }
            }
            _ => {}
        }

        PacketAction::Allow
    }
}

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