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

pub fn read_varlong(buf: &[u8]) -> Option<(i64, usize)> {
    let mut num: i64 = 0;
    let mut shift = 0;

    for (i, byte) in buf.iter().enumerate() {
        // leer últimos 7 bits
        let val = (byte & 0b01111111) as i64;
        num |= val << shift;

        // si el primer bit es 0, es el ultimo byte
        if byte & 0b10000000 == 0 {
            return Some((num, i + 1));
        }

        shift += 7;
        if shift >= 64 {
            return None;
        }
    }

    None
}

pub fn write_varint(value: i32, buf: &mut Vec<u8>) {
    let mut u_value = value as u32;

    loop {
        let mut byte = (u_value & 0b01111111) as u8;
        u_value >>= 7;
        if u_value != 0 {
            byte |= 0b10000000;
        }
        buf.push(byte);
        if u_value == 0 {
            break;
        }
    }
}

pub fn write_varlong(value: i64, buf: &mut Vec<u8>) {
    let mut u_value = value as u64;

    loop {
        let mut byte = (u_value & 0b01111111) as u8;
        u_value >>= 7;
        if u_value != 0 {
            byte |= 0b10000000;
        }
        buf.push(byte);
        if u_value == 0 {
            break;
        }
    }
}

pub fn read_string(buf: &[u8]) -> Option<(String, usize)> {
    let (length, len_size) = read_varint(buf)?;
    let total_size = len_size + length as usize;

    if buf.len() < total_size {
        return None;
    }

    let string_bytes = &buf[len_size..total_size];
    return Some((String::from_utf8_lossy(string_bytes).to_string(), total_size));
}

pub fn write_string(text: &str, buf: &mut Vec<u8>) {
    let length = text.len() as i32;

    write_varint(length, buf);
    buf.extend_from_slice(text.as_bytes());
}

pub fn read_ushort(buf: &[u8]) -> Option<(u16, usize)> {
    if buf.len() < 2 {
        return None;
    }
    let n = u16::from_be_bytes([buf[0], buf[1]]);
    Some((n, 2))
}

pub fn write_ushort(value: u16, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&u16::to_be_bytes(value));
}

pub fn read_long(buf: &[u8]) -> Option<(i64, usize)> {
    if buf.len() < 8 {
        return None;
    }
    let bytes: [u8; 8] = buf[..8].try_into().unwrap();
    let n = i64::from_be_bytes(bytes);
    Some((n, 8))
}

pub fn write_long(value: i64, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&i64::to_be_bytes(value));
}

#[cfg(test)]
mod tests {
    use crate::protocol::varint::{read_string, read_varint, read_varlong, write_string, write_varint, write_varlong};

    #[test]
    fn test_read_single_byte() {
        let data = [0x05];
        let (val, size) = read_varint(&data).unwrap();
        assert_eq!(val, 5);
        assert_eq!(size, 1)
    }

    #[test]
    fn test_read_multi_byte() {
        let data = [0xff, 0xff, 0x7f];
        let (val, size) = read_varint(&data).unwrap();
        assert_eq!(val, 2097151);
        assert_eq!(size, 3);
    }

    #[test]
    fn test_incomplete_varint() {
        // Si el primer bit es 1 pero no hay más bytes, debe dar None
        let data = [0x80]; 
        assert!(read_varint(&data).is_none());
    }

    #[test]
    fn test_write_varint() {
        let mut buf: Vec<u8> = Vec::new();
        write_varint(2097151, &mut buf);
        assert_eq!(buf, vec![0xff, 0xff, 0x7f])
    }

    #[test]
    fn read_write_varlong() {
        let mut buf: Vec<u8> = Vec::new();
        write_varlong(12345678910, &mut buf);
        let (val, size) = read_varlong(&buf).unwrap();
        assert_eq!(val, 12345678910);
        assert_eq!(size, 5)
    }

    #[test]
    fn test_string_codec() {
        let mut buf = Vec::new();
        write_string("Test", &mut buf);

        let (result, size) = read_string(&buf).unwrap();
        assert_eq!(result, "Test");
        assert_eq!(size, 5); // 1 byte de varint + 4 de texto
    }
}
