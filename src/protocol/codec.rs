use crate::protocol::{Direction, FilterResult, PacketReader};
use crate::protocol::packets::*;
use crate::protocol::{read_varint};

use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn inspect_packet(
    buffer: &mut Vec<u8>,
    dir: &Direction,
    session: &mut Session
) -> FilterResult {
    // Leer largo total del paquete
    let (total_length, len_size) = match read_varint(&buffer) {
        Some(v) => v,
        None => return FilterResult::Incomplete
    };

    let total_packet_size = len_size + total_length as usize;
    if buffer.len() < total_packet_size {
        return FilterResult::Incomplete
    }

    let raw_packet = buffer.drain(..total_packet_size).collect::<Vec<u8>>();

    // Decompresion
    let final_id: i32;
    let final_data: Vec<u8>;

    let mut offset = len_size;

    if session.compression_threshold >= 0 {
        let (data_length, data_len_size) = match read_varint(&raw_packet[offset..]) {
            Some(v) => v,
            None => return FilterResult::Cancel
        };
        offset += data_len_size;

        if data_length == 0 {
            // data_length es 0 cuando no está comprimido
            let (id, id_size) = match read_varint(&raw_packet[offset..]) {
                Some(v) => v,
                None => return FilterResult::Cancel
            };
            offset += id_size;

            final_id = id;
            final_data = raw_packet[offset..].to_vec()
        } else {
            let mut decoder = ZlibDecoder::new(&raw_packet[offset..]);
            let mut decompressed = Vec::new();
            if decoder.read_to_end(&mut decompressed).is_err() {
                return FilterResult::Cancel
            }

            let (id, id_size) = match read_varint(&decompressed) {
                Some(v) => v,
                None => return FilterResult::Cancel
            };

            final_id = id;
            final_data = decompressed[id_size..].to_vec()
        }
    } else {
        let (id, id_size) = match read_varint(&raw_packet[offset..]) {
            Some(v) => v,
            None => return FilterResult::Cancel
        };
        offset += id_size;

        final_id = id;
        final_data = raw_packet[offset..].to_vec()
    }

    let packet = Packet {
        id: final_id,
        data: final_data,
    };

    let mut reader = PacketReader::new(&packet.data);
    let state = &mut session.state;

    match dir {
        Direction::ClientToServer => match state {
            ConnectionState::Handshaking    => HandshakeHandler::handle_c2s(&mut reader, packet.id, session),
            ConnectionState::Status         => StatusHandler::handle_c2s(&mut reader, packet.id, session),
            ConnectionState::Login          => LoginHandler::handle_c2s(&mut reader, packet.id, session),
            ConnectionState::Configuration  => ConfigurationHandler::handle_c2s(&mut reader, packet.id, session),
            ConnectionState::Play           => PlayHandler::handle_c2s(&mut reader, packet.id, session),
            _ => {}
        }
        Direction::ServerToClient => match state {
            ConnectionState::Handshaking    => HandshakeHandler::handle_s2c(&mut reader, packet.id, session),
            ConnectionState::Status         => StatusHandler::handle_s2c(&mut reader, packet.id, session),
            ConnectionState::Login          => LoginHandler::handle_s2c(&mut reader, packet.id, session),
            ConnectionState::Configuration  => ConfigurationHandler::handle_s2c(&mut reader, packet.id, session),
            ConnectionState::Play           => PlayHandler::handle_s2c(&mut reader, packet.id, session),
            _ => {}
        }
    }

    FilterResult::Send(raw_packet)
}