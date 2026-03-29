// https://minecraft.wiki/w/Java_Edition_protocol/Packets#VarInt_and_VarLong
pub fn read_varint(buf: &[u8]) -> Option<(i32, usize)> {
    let mut num = 0;
    let mut shift = 0;

    for (i, byte) in buf.iter().enumerate() {
        // leer últimos 7 bits
        let val = (byte & 0b01111111) as i32;
        num |= val << shift;

        // si el primer bit es 0, es el ultimo byte
        if byte & 0b10000000 == 0 {
            return Some((num, i + 1));
        }

        shift += 7;
        if shift >= 32 {
            return None;
        }
    }

    None
}