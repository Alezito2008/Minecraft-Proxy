use crate::impl_packet_handler;
use crate::protocol::listener::{PacketAction, PacketListener};
use crate::protocol::{ConnectionState, PacketReader, Session};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login
impl_packet_handler!(LoginHandler, packet, session,
    c2s => {
        LoginStart => on_login_start
        EncryptionResponse => on_encryption_response
        LoginAcknowledged => on_login_acknowledged, { session.state = ConnectionState::Configuration }
    }
    s2c => {
        EncryptionRequest => on_encryption_request
        SetCompression => on_set_compression, { session.compression_threshold = packet.threshold }
        LoginSuccess => on_login_success
    }
);

pub mod packets {
    use super::*;
    use crate::protocol::varint::*;
    
    pub struct LoginStart {
        pub name: String,
        pub uuid: u128,
    }

    impl MinecraftPacket for LoginStart {
        const ID: i32 = 0x00;

        fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                name: reader.read_string()?,
                uuid: reader.read_uuid()?,
            })
        }

        fn encode(&self, buf: &mut Vec<u8>) {
            write_string(&self.name, buf);
            write_uuid(self.uuid, buf);
        }
    }

    pub struct EncryptionRequest; // TODO

    impl MinecraftPacket for EncryptionRequest {
        const ID: i32 = 0x01;
    }

    pub struct EncryptionResponse; // TODO

    impl MinecraftPacket for EncryptionResponse {
        const ID: i32 = 0x01;
    }

    pub struct SetCompression {
        pub threshold: i32
    }

    impl MinecraftPacket for SetCompression {
        const ID: i32 = 0x03;

        fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                threshold: reader.read_varint()?,
            })
        }

        fn encode(&self, buf: &mut Vec<u8>) {
            write_varint(self.threshold, buf);
        }
    }

    pub struct LoginSuccess {
        pub uuid: u128,
        pub username: String,
        // TODO properties
    }

    impl MinecraftPacket for LoginSuccess {
        const ID: i32 = 0x02;

        fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                uuid: reader.read_uuid()?,
                username: reader.read_string()?,
                // TODO properties
            })
        }

        fn encode(&self, buf: &mut Vec<u8>) {
            write_uuid(self.uuid, buf);
            write_string(&self.username, buf);
            // TODO properties
        }
    }

    pub struct LoginAcknowledged;

    impl MinecraftPacket for LoginAcknowledged {
        const ID: i32 = 0x03;
    }
}