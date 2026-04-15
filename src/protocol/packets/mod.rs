mod handshaking;
mod login;
mod status;
mod configuration;
mod play;

pub use handshaking::packets::*;
pub use status::packets::*;
pub use login::packets::*;
pub use configuration::packets::*;
pub use play::packets::*;

pub use handshaking::HandshakeHandler;
pub use status::StatusHandler;
pub use login::LoginHandler;
pub use configuration::ConfigurationHandler;
pub use play::PlayHandler;

use crate::protocol::{PacketReader, listener::{PacketAction, PacketListener}};

pub struct Session {
    pub state: ConnectionState,
    pub compression_threshold: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Transfer,
    Configuration,
    Play,
    Unknown,
}

impl From<i32> for ConnectionState {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Status,
            2 => Self::Login,
            3 => Self::Transfer,
            _ => Self::Unknown
        }
    }
}

impl From<ConnectionState> for i32 {
    fn from(value: ConnectionState) -> Self {
        match value {
            ConnectionState::Status => 1,
            ConnectionState::Login  => 2,
            ConnectionState::Transfer => 3,
            _ => -1
        }
    }
}

#[allow(unused)]
pub trait MinecraftPacket {
    const ID: i32;
    fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized { None }
    fn encode(&self, buf: &mut Vec<u8>) {}
}

pub struct Packet {
    pub id: i32,
    pub data: Vec<u8>,
}

pub trait PacketHandler {
    fn handle_c2s<L: PacketListener>(
        _reader: &mut PacketReader,
        _id: i32,
        _session: &mut Session,
        _listener: &mut L
    ) -> PacketAction { PacketAction::Allow }

    fn handle_s2c<L: PacketListener>(
        _reader: &mut PacketReader,
        _id: i32,
        _session: &mut Session,
        _listener: &mut L
    ) -> PacketAction { PacketAction::Allow }
}

#[macro_export]
macro_rules! handle_packets {
    ($id:ident, $reader:ident, $listener:ident, $session:ident; $($packet_type:ty => $packet_listener:ident $(, $p_name:ident, $e:expr)?);+ $(;)?) => {
        {
            match $id {
                $(
                    <$packet_type>::ID => {
                        if let Some(mut _p) = <$packet_type>::decode($reader) {
                            $(
                                #[allow(unused)]
                                let $p_name = &mut _p;
                                $e;
                            )?
                            return $listener.$packet_listener(&mut _p)
                        }
                    }
                )+
                _ => ()
            };

            PacketAction::Allow
        }
    };
}

#[macro_export]
macro_rules! impl_packet_handler {
    ($name:ident, $p_name:ident, $session:ident,
        $(c2s => { $($c2s_packet_type:ty => $c2s_packet_listener:ident $(, $c2s_e:expr)?)* })? $(,)?
        $(s2c => { $($s2c_packet_type:ty => $s2c_packet_listener:ident $(, $s2c_e:expr)?)* })?
    ) => {
        pub struct $name;
        impl PacketHandler for $name {
            $(
                #[allow(unused)]
                fn handle_c2s<L: PacketListener>(
                    reader: &mut PacketReader,
                    id: i32,
                    $session: &mut Session,
                    listener: &mut L
                ) -> PacketAction {
                    crate::handle_packets!(id, reader, listener, $session;
                        $($c2s_packet_type => $c2s_packet_listener $(, $p_name, $c2s_e)?;)*
                    )
                }
            )?
            $(
                #[allow(unused)]
                fn handle_s2c<L: PacketListener>(
                    reader: &mut PacketReader,
                    id: i32,
                    $session: &mut Session,
                    listener: &mut L
                ) -> PacketAction {
                    crate::handle_packets!(id, reader, listener, session;
                        $($s2c_packet_type => $s2c_packet_listener $(, $p_name, $s2c_e)?;)*
                    )
                }
            )?
        }
    };
}
