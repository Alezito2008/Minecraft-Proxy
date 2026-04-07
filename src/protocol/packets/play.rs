use crate::protocol::listener::{PacketAction, PacketListener};
use crate::protocol::{PacketReader, Session, ConnectionState};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use crate::protocol::types::EntityType;
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play
pub struct PlayHandler;
impl PacketHandler for PlayHandler {
    fn handle_c2s<L: PacketListener>(
            reader: &mut PacketReader,
            id: i32,
            session: &mut Session,
            listener: &mut L
        ) -> PacketAction {
        match id {
            ChatCommand::ID => {
                if let Some(mut chat_command) = ChatCommand::decode(reader) {
                    println!("Sent chat command: {}", chat_command.command);
                    return listener.on_chat_command(&mut chat_command);
                }
            }
            CommandSuggestionRequest::ID => {
                println!("cmd suggestion");
                if let Some(mut command_suggestion) = CommandSuggestionRequest::decode(reader) {
                    println!("Command suggestion: ID: {}, Text: {}", command_suggestion.transaction_id, command_suggestion.text);
                    return listener.on_command_suggestion_request(&mut command_suggestion);
                }
            }
            AcknowledgeConfiguration::ID => {
                println!("Acknowledge start configuration");
                session.state = ConnectionState::Configuration;
                return listener.on_acknowledge_configuration(&mut AcknowledgeConfiguration);
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
            StartConfiguration::ID => {
                println!("Start configuration request");
                return listener.on_start_configuration(&mut StartConfiguration);
            }
            SpawnEntity::ID => {
                if let Some(mut e) = SpawnEntity::decode(reader) {
                    println!("Spawned entity type: {:?}, ID: {}, x: {}, y: {}, z: {}",
                        e.entity_type,
                        e.entity_id,
                        e.x, e.y, e.z
                    );
                    return listener.on_spawn_entity(&mut e);
                }
            }
            EntityPositionSync::ID => {
                if let Some(mut e) = EntityPositionSync::decode(reader) {
                    println!("Teleported entity with ID: {} at x: {}, y: {}, z: {}",
                        e.entity_id,
                        e.x, e.y, e.z
                    );
                    return listener.on_entity_position_sync(&mut e);
                }
            }
            UpdateEntityPosition::ID => {
                if let Some(mut e) = UpdateEntityPosition::decode(reader) {
                    println!("Update entity position for ID: {}: dx: {}, dy: {}, dz: {}",
                        e.entity_id,
                        e.dx, e.dy, e.dz
                    );
                    return listener.on_update_entity_position(&mut e);
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

    pub struct EntityPositionSync {
        pub entity_id: i32,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub vel_x: f64,
        pub vel_y: f64,
        pub vel_z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool
    }

    impl MinecraftPacket for EntityPositionSync {
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

    pub struct SpawnEntity {
        pub entity_id: i32,
        pub uuid: u128,
        pub entity_type: EntityType,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        // TODO: Completar
    }

    impl MinecraftPacket for SpawnEntity {
        const ID: i32 = 0x01;

        fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                entity_id: reader.read_varint()?,
                uuid: reader.read_uuid()?,
                entity_type: EntityType::from(reader.read_varint()?),
                x: reader.read_double()?,
                y: reader.read_double()?,
                z: reader.read_double()?
            })
        }
    }

    pub struct UpdateEntityPosition {
        pub entity_id: i32,
        pub dx: i16,
        pub dy: i16,
        pub dz: i16,
        pub on_ground: bool
    }

    impl MinecraftPacket for UpdateEntityPosition {
        const ID: i32 = 0x33;

        fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                entity_id: reader.read_varint()?,
                dx: reader.read_short()?,
                dy: reader.read_short()?,
                dz: reader.read_short()?,
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
