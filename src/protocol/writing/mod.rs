#![allow(dead_code)]

use std;

use protocol::identification;


pub fn write_magic<W: std::io::Write>(w: &mut W) -> std::io::Result<()> {
    w.write_all(b"  V2")
}

//OK
//E_INVALID
//E_BAD_BODY
pub fn write_identify<W: std::io::Write>(w: &mut W, identification: identification::Identification) -> std::io::Result<()> {
    let identification_string: String = identification.into();
    let identification_bytes = identification_string.as_bytes();
    if identification_bytes.len() > u32::max_value() as usize {
        panic!("");
    }
    let length: u32 = (identification_bytes.len() as u32).to_be();
    let length: [u8; 4] = unsafe { std::mem::transmute(length) };
    try!(w.write(b"IDENTIFY\n"));
    try!(w.write(&length));
    w.write(identification_bytes).map(|_| ())
}

// OK
// E_INVALID
// E_BAD_TOPIC
// E_BAD_CHANNEL
pub fn write_sub<W: std::io::Write>(w: &mut W, topic: &str, channel: &str) -> std::io::Result<()> {
    write!(w, "SUB {} {}\n", topic, channel)
}

// OK
// E_INVALID
// E_BAD_TOPIC
// E_BAD_MESSAGE
// E_PUB_FAILED
pub fn write_pub<W: std::io::Write>(w: &mut W, topic: &str, message: &[u8]) -> std::io::Result<()> {
    try!(write!(w, "PUB {}\n", topic));
    if message.len() > u32::max_value() as usize {
        panic!("");
    }
    let length: u32 = (message.len() as u32).to_be();
    let length: [u8; 4] = unsafe { std::mem::transmute(length) };
    try!(w.write(&length));
    w.write(message).map(|_| ())
}

// OK
// E_INVALID
// E_BAD_TOPIC
// E_BAD_BODY
// E_BAD_MESSAGE
// E_MPUB_FAILED
// MPUB
pub fn write_mpub<'m, M, W: std::io::Write>(w: &mut W, topic: &str, messages: M) -> std::io::Result<()>
    where M: IntoIterator<Item=&'m [u8],IntoIter=Iterator<Item=&'m [u8]>>
{
    let _ = topic;
    let _ = messages;
    let _ = w;
    Ok(())
}

// NO SUCCESS
// E_INVALID
pub fn write_rdy<W: std::io::Write>(w: &mut W, rdy: u32) -> std::io::Result<()> {
    write!(w, "RDY {}\n", rdy)
}

// NO SUCCESS
// E_INVALID
// E_FIN_FAILED
pub fn write_fin<W: std::io::Write>(w: &mut W, message_id: &[u8]) -> std::io::Result<()> {
    // println!("FIN: {}", String::from_utf8(message_id.to_owned()).unwrap());
    try!(w.write(b"FIN "));
    try!(w.write(message_id));
    try!(w.write(b"\n"));
    Ok(())
}

// NO SUCCESS
// E_INVALID
// E_REQ_FAILED
pub fn write_req<W: std::io::Write>(w: &mut W, message_id: &str, timeout: u32) -> std::io::Result<()> {
    write!(w, "REQ {} {}\n", message_id, timeout)
}

// NO SUCCESS
// E_INVALID
// E_TOUCH_FAILED
pub fn write_touch<W: std::io::Write>(w: &mut W, message_id: &str) -> std::io::Result<()> {
    write!(w, "TOUCH {}\n", message_id)
}

// CLOSE_WAIT
// E_INVALID
pub fn write_cls<W: std::io::Write>(w: &mut W) -> std::io::Result<()> {
    write!(w, "CLS\n")
}

// NO SUCCESS
pub fn write_nop<W: std::io::Write>(w: &mut W) -> std::io::Result<()> {
    write!(w, "NOP\n")
}

pub fn write_auth<W: std::io::Write>(w: &mut W, secret: &[u8]) -> std::io::Result<()> {
    try!(write!(w, "AUTH\n"));
    if secret.len() > u32::max_value() as usize {
        panic!("");
    }
    let length: u32 = (secret.len() as u32).to_be();
    let length: [u8; 4] = unsafe { std::mem::transmute(length) };
    try!(w.write(&length));
    w.write(secret).map(|_| ())
}


#[cfg(test)]
mod test {
    use std;
    use super::*;
    use protocol::identification;

    #[test]
    fn smoke() {
        let mut b: Vec<u8> = Vec::new();

        {
            let mut c = std::io::Cursor::new(b);
            write_magic(&mut c).unwrap();
            let i = identification::Identification::builder().build();
            write_identify(&mut c, i).unwrap();
            write_sub(&mut c, "topicname", "channelname").unwrap();
            write_pub(&mut c, "othertopicname", "{\"foo\":\"bar\"}".as_bytes()).unwrap();
            write_rdy(&mut c, 2).unwrap();
            b = c.into_inner();
        }

        let mut i = 0;
        assert_eq!(&b[i..i+4], b"  V2"); i += 4;
        assert_eq!(&b[i..i+13], b"IDENTIFY\n\x00\x00\x00\x1C"); i += 13;
        assert_eq!(&b[i..i+28], b"{\"feature_negotiation\":true}"); i += 28;
        assert_eq!(&b[i..i+26], b"SUB topicname channelname\n"); i += 26;
        assert_eq!(&b[i..i+19], b"PUB othertopicname\n"); i += 19;
        assert_eq!(&b[i..i+17], b"\x00\x00\x00\x0D{\"foo\":\"bar\"}"); i += 17;
        assert_eq!(&b[i..i+6], b"RDY 2\n"); i += 6;
        assert_eq!(i, 4 + 13 + 28 + 26 + 19 + 17 + 6);
        assert_eq!(b.len(), i);
    }
}
