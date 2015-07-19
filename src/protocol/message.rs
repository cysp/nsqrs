use std;

use error::Error;
use result::Result;


fn from_hex_nybble(b: u8) -> Option<u8> {
    if b >= ('0' as u8) && b <= ('9' as u8) {
        return Some(b - ('0' as u8));
    }
    if b >= ('A' as u8) && b <= ('F' as u8) {
        return Some(10 + b - ('A' as u8));
    }
    if b >= ('a' as u8) && b <= ('f' as u8) {
        return Some(10 + b - ('a' as u8));
    }
    None
}

fn from_hex(b: &[u8]) -> Option<u8> {
    if b.len() != 2 {
        return None;
    }
    let high_nybble = match from_hex_nybble(b[0]) {
        Some(v) => v,
        None => return None,
    };
    let low_nybble = match from_hex_nybble(b[1]) {
        Some(v) => v,
        None => return None,
    };
    Some(high_nybble << 4 | low_nybble)
}


#[derive(Copy,Clone,Debug,PartialEq,PartialOrd)]
pub struct MessageId {
    id: u64,
}

impl MessageId {
    pub fn from_hex_bytes(hb: &[u8]) -> Result<MessageId> {
        if hb.len() != 16 {
            return Err(Error::invalid_message_id_error(hb));
        }

        let mut b = [0u8; 8];
        for (hb, b) in hb.chunks(2).zip(b.iter_mut()) {
            match from_hex(hb) {
                Some(v) => *b = v,
                None => return Err(Error::invalid_message_id_error(hb)),
            }
        }

        Ok(MessageId{ id: u64::from_be(unsafe { std::mem::transmute(b) }) })
    }

    pub fn to_hex_bytes(&self) -> Vec<u8> {
        format!("{:016x}", self.id).into()
    }

    pub fn as_u64(&self) -> u64 {
        self.id
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        let m = MessageId::from_hex_bytes("anobviouslyinvalidmessageid".as_bytes());
        assert!(m.is_err());

        let m = MessageId::from_hex_bytes("0000000000000000".as_bytes()).unwrap();

        assert_eq!(m.as_u64(), 0);
        assert_eq!(m.to_hex_bytes(), "0000000000000000".as_bytes());

        let m = MessageId::from_hex_bytes("1234567890abcdef".as_bytes()).unwrap();

        assert_eq!(m.as_u64(), 0x1234567890abcdef);
        assert_eq!(String::from_utf8(m.to_hex_bytes()).unwrap(), "1234567890abcdef");
    }
}
