use crate::protocol::listener::{PacketAction, PacketListener};
use crate::protocol::{ConnectionState, PacketReader, Session};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login
pub struct LoginHandler;
impl PacketHandler for LoginHandler {
    fn handle_c2s<L: PacketListener>(
            reader: &mut PacketReader,
            id: i32,
            session: &mut Session,
            listener: &mut L
        ) -> PacketAction {
        match id {
            LoginStart::ID => {
                if let Some(mut login_start) = LoginStart::decode(reader) {
                    println!("Login start: Name: {}, UUID: {}", login_start.name, login_start.uuid);
                    return listener.on_login_start(&mut login_start);
                }
            },
            EncryptionResponse::ID => {
                println!("Sent encryption response packet");
                return listener.on_encription_response(&mut EncryptionResponse)
            },
            LoginAcknowledged::ID => {
                println!("Login acknowledged");
                session.state = ConnectionState::Configuration;
                return listener.on_login_acknowledged(&mut LoginAcknowledged)
            }
            _ => {}
        }
        PacketAction::Allow
    }

    fn handle_s2c<L: PacketListener>(
            reader: &mut PacketReader,
            id: i32,
            session: &mut Session,
            listener: &mut L
        ) -> PacketAction {
        
        match id {
            EncryptionRequest::ID => {
                println!("Received encryption request packet");
                return listener.on_encryption_request(&mut EncryptionRequest);
            },
            SetCompression::ID => {
                if let Some(mut set_compression) = SetCompression::decode(reader) {
                    println!("Set compression threshold: {}", set_compression.threshold);
                    session.compression_threshold = set_compression.threshold;
                    return listener.on_set_compression(&mut set_compression);
                }
            }
            LoginSuccess::ID => {
                if let Some(mut login_success) = LoginSuccess::decode(reader) {
                    println!("Login success: UUID: {}, Username: {}", login_success.uuid, login_success.username);
                    return listener.on_login_success(&mut login_success)
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