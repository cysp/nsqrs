use std;

use error::Error;
use result::Result;

use protocol;
use protocol::identification;


#[derive(Clone,Debug,PartialEq)]
pub enum StringOrByteVec {
    String(String),
    ByteVec(Vec<u8>),
}

impl From<String> for StringOrByteVec {
    fn from(s: String) -> StringOrByteVec {
        StringOrByteVec::String(s)
    }
}

impl From<Vec<u8>> for StringOrByteVec {
    fn from(v: Vec<u8>) -> StringOrByteVec {
        match String::from_utf8(v) {
            Ok(string) => StringOrByteVec::String(string),
            Err(e) => StringOrByteVec::ByteVec(e.into_bytes()),
        }
    }
}

impl From<StringOrByteVec> for Vec<u8> {
    fn from(v: StringOrByteVec) -> Vec<u8> {
        match v {
            StringOrByteVec::String(s) => s.into(),
            StringOrByteVec::ByteVec(v) => v,
        }
    }
}


#[derive(Clone,Debug,PartialEq)]
pub enum ResponseFrame {
    Heartbeat,
    Ok,
    CloseWait,
    Data(StringOrByteVec),
}
impl ResponseFrame {
    pub fn from_vec(v: Vec<u8>) -> ResponseFrame {
        if v == "_heartbeat_".as_bytes() {
            return ResponseFrame::Heartbeat;
        }
        if v == "OK".as_bytes() {
            return ResponseFrame::Ok;
        }
        if v == "CLOSE_WAIT".as_bytes() {
            return ResponseFrame::CloseWait;
        }
        ResponseFrame::Data(v.into())
    }
}


#[derive(Clone,Debug,PartialEq)]
pub enum ErrorFrame {
    Invalid(Option<StringOrByteVec>),
    BadBody(Option<StringOrByteVec>),
    BadTopic(Option<StringOrByteVec>),
    // BadChannel,
    // BadMessage,
    // PubFailed,
    // MpubFailed,
    // FinFailed,
    // ReqFailed,
    // TouchFailed,
    // AuthFailed,
    // Unauthorized,
    Unknown(StringOrByteVec),
}
impl ErrorFrame {
    pub fn from_vec(v: Vec<u8>) -> ErrorFrame {
        if &v[0..10] == "E_INVALID ".as_bytes() {
            return ErrorFrame::Invalid(Some((&v[10..]).to_owned().into()));
        }
        if v == "E_INVALID".as_bytes() {
            return ErrorFrame::Invalid(None);
        }
        //     E_BAD_BODY,
        //     E_BAD_TOPIC,
        //     E_BAD_CHANNEL,
        //     E_BAD_MESSAGE,
        //     E_PUB_FAILED,
        //     E_MPUB_FAILED,
        //     E_FIN_FAILED,
        //     E_REQ_FAILED,
        //     E_TOUCH_FAILED,
        //     E_AUTH_FAILED,
        //     E_UNAUTHORIZED,
        if v == "E_BAD_BODY".as_bytes() {
            return ErrorFrame::BadBody(None);
        }
        if v == "E_BAD_TOPIC".as_bytes() {
            return ErrorFrame::BadTopic(None);
        }
        ErrorFrame::Unknown(v.into())
    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct MessageFrame {
    pub timestamp: i64,
    pub attempts: u16,
    pub message_id: protocol::message::MessageId,
    pub message: StringOrByteVec,
}
impl MessageFrame {
    pub fn from_vec(v: Vec<u8>) -> Result<MessageFrame> {
        if v.len() < 8 + 2 + 16 {
            return Err(Error::invalid_message_frame_error());
        }

        println!("messageframe: {:?}", v);

        let mut timestamp = [0u8; 8];
        for (&x, p) in v[0..8].iter().zip(timestamp.iter_mut()) {
            *p = x;
        }
        let timestamp = i64::from_be(unsafe { std::mem::transmute(timestamp) });

        let mut attempts = [0u8; 2];
        for (&x, p) in v[8..10].iter().zip(attempts.iter_mut()) {
            *p = x;
        }
        let attempts = u16::from_be(unsafe { std::mem::transmute(attempts) });

        let message_id = try!(protocol::message::MessageId::from_hex_bytes(&v[10..26]));

        let message = v[26..].to_owned();

        Ok(MessageFrame {
            timestamp: timestamp,
            attempts: attempts,
            message_id: message_id,
            message: message.into(),
        })
    }
}


#[derive(Clone,Debug,PartialEq)]
pub enum Frame {
    ResponseFrame(ResponseFrame),
    ErrorFrame(ErrorFrame),
    MessageFrame(MessageFrame),
}

pub struct Connection {
    conn: std::net::TcpStream,
}

impl Connection {
    pub fn connect(addr: std::net::SocketAddr) -> Result<Connection> {
        let mut conn = Connection {
            conn: try!(std::net::TcpStream::connect(addr)),
        };

        try!(conn.send_magic());
        Ok(conn)
    }

    fn send_magic(&mut self) -> Result<()> {
        try!(protocol::writing::write_magic(&mut self.conn));
        Ok(())
    }

    pub fn send_identify(&mut self, identification: identification::Identification) -> Result<()> {
        try!(protocol::writing::write_identify(&mut self.conn, identification));
        Ok(())
    }

    pub fn send_sub(&mut self, topic: &str, channel: &str) -> Result<()> {
        try!(protocol::writing::write_sub(&mut self.conn, topic, channel));
        Ok(())
    }

    pub fn send_rdy(&mut self, rdy: u32) -> Result<()> {
        try!(protocol::writing::write_rdy(&mut self.conn, rdy));
        Ok(())
    }

    pub fn send_fin(&mut self, message_id: protocol::message::MessageId) -> Result<()> {
        try!(protocol::writing::write_fin(&mut self.conn, &message_id.to_hex_bytes()));
        Ok(())
    }

    pub fn send_cls(&mut self) -> Result<()> {
        try!(protocol::writing::write_cls(&mut self.conn));
        Ok(())
    }

    pub fn send_nop(&mut self) -> Result<()> {
        try!(protocol::writing::write_nop(&mut self.conn));
        Ok(())
    }

    pub fn recv_frame(&mut self) -> Result<Frame> {
        let mut r = protocol::reading::ProtocolReader::new(&mut self.conn);
        match try!(r.read_frame()) {
            (protocol::reading::FrameType::Response, data) => {
                Ok(Frame::ResponseFrame(ResponseFrame::from_vec(data)))
            }
            (protocol::reading::FrameType::Error, data) => {
                Ok(Frame::ErrorFrame(ErrorFrame::from_vec(data)))
            }
            (protocol::reading::FrameType::Message, data) => {
                let frame = try!(MessageFrame::from_vec(data));
                Ok(Frame::MessageFrame(frame))
            }
            (protocol::reading::FrameType::Unknown(frame_type), _) => Err(Error::unknown_frame_type_error(frame_type)),
        }
    }

    // pub fn
}


#[cfg(unix)]
impl std::os::unix::io::AsRawFd for Connection {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.conn.as_raw_fd()
    }
}
