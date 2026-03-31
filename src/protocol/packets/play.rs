use crate::protocol::{PacketReader, Session};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play
pub struct PlayHandler;
impl PacketHandler for PlayHandler {
    fn handle_c2s(reader: &mut PacketReader, id: i32, _session: &mut Session) {
        match id {
            ChatCommand::ID => {
                if let Some(chat_command) = ChatCommand::decode(reader) {
                    println!("Sent chat command: {}", chat_command.command);
                }
            }
            _ => {}
        }
    }

    fn handle_s2c(_reader: &mut PacketReader, _id: i32, _session: &mut Session) {}
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
}
