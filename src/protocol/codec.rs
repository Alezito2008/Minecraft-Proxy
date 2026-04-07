use crate::protocol::listener::PacketListener;
use crate::protocol::{Direction, FilterResult, PacketReader};
use crate::protocol::packets::*;
use crate::protocol::{read_varint};

use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn deconstruct_packet(payload: &[u8], threshold: i32) -> Option<(i32, Vec<u8>)> {
    let mut offset = 0;

    if threshold >= 0 {
        let (data_length, data_len_size) = read_varint(&payload[offset..])?;
        offset += data_len_size;

        if data_length == 0 {
            // data_length es 0 cuando no está comprimido
            let (id, id_size) = read_varint(&payload[offset..])?;
            offset += id_size;

            let data = payload[offset..].to_vec();
            Some((id, data))
        } else {
            // paquete
            let mut decoder = ZlibDecoder::new(&payload[offset..]);
            let mut decompressed = Vec::with_capacity(data_length as usize);
            decoder.read_to_end(&mut decompressed).ok()?;

            let (id, id_size) = read_varint(&decompressed)?;
            let data= decompressed[id_size..].to_vec();

            Some((id, data))
        }
    } else {
        let (id, id_size) = read_varint(&payload[offset..])?;
        offset += id_size;

        let data = payload[offset..].to_vec();
        Some((id, data))
    }
}

pub fn inspect_packet<L: PacketListener>(
    buffer: &mut Vec<u8>,
    dir: &Direction,
    session: &mut Session,
    listener: &mut L
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

    let payload = &raw_packet[len_size..];
    let (final_id, final_data) = match deconstruct_packet(payload, session.compression_threshold) {
        Some(v) => v,
        None => return FilterResult::Cancel
    };

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