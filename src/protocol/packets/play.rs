use crate::protocol::{PacketReader, Session, ConnectionState};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play
pub struct PlayHandler;
impl PacketHandler for PlayHandler {
    fn handle_c2s(reader: &mut PacketReader, id: i32, session: &mut Session) {
        match id {
            ChatCommand::ID => {
                if let Some(chat_command) = ChatCommand::decode(reader) {
                    println!("Sent chat command: {}", chat_command.command);
                }
            }
            CommandSuggestionRequest::ID => {
                println!("cmd suggestion");
                if let Some(command_suggestion) = CommandSuggestionRequest::decode(reader) {
                    println!("Command suggestion: ID: {}, Text: {}", command_suggestion.transaction_id, command_suggestion.text);
                }
            }
            AcknowledgeConfiguration::ID => {
                println!("Acknowledge start configuration");
                session.state = ConnectionState::Configuration;
            }
            _ => {}
        }
    }

    fn handle_s2c(_reader: &mut PacketReader, id: i32, _session: &mut Session) {
        match id {
            StartConfiguration::ID => {
                println!("Start configuration request");
            }
            _ => {}
        }
    }
}

pub mod packets {
    use super::*;
    use crate::protocol::varint::*;

    pub struct ChatCommand {
        pub command: String
    }

    impl MinecraftPacket for ChatCommand {
        const ID: i32 = 0x06;

        fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                command: reader.read_string()?
            })
        }

        fn encode(&self, buf: &mut Vec<u8>) {
            write_string(&self.command, buf);
        }
    }

    pub struct CommandSuggestionRequest {
        pub transaction_id: i32,
        pub text: String
    }

    impl MinecraftPacket for CommandSuggestionRequest {
        const ID: i32 = 0x0E;

        fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                transaction_id: reader.read_varint()?,
                text: reader.read_string()?
            })
        }

        fn encode(&self, buf: &mut Vec<u8>) {
            write_varint(self.transaction_id, buf);
            write_string(&self.text, buf);
        }
    }

    pub struct TeleportEntity {
        entity_id: i32,
        x: f64,
        y: f64,
        z: f64,
        vel_x: f64,
        vel_y: f64,
        vel_z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool
    }

    impl MinecraftPacket for TeleportEntity {
        const ID: i32 = 0x23;

        fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                entity_id: reader.read_varint()?,
                x: reader.read_double()?,
                y: reader.read_double()?,
                z: reader.read_double()?,
                vel_x: reader.read_double()?,
                vel_y: reader.read_double()?,
                vel_z: reader.read_double()?,
                yaw: reader.read_float()?,
                pitch: reader.read_float()?,
                on_ground: reader.read_bool()?
            })
        }
    }

    pub struct StartConfiguration;

    impl MinecraftPacket for StartConfiguration {
        const ID: i32 = 0x74;
    }

    pub struct AcknowledgeConfiguration;

    impl MinecraftPacket for AcknowledgeConfiguration {
        const ID: i32 = 0x0F;
    }
}
