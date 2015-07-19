use std;


pub struct ProtocolReader<'r, R> where R: std::io::Read + 'r {
    r: &'r mut R,
    buf: Vec<u8>,
    // write_cur: Cursor<Vec<u8>>,
}


#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum FrameType {
    Response,
    Error,
    Message,
    Unknown(u32),
}

impl From<u32> for FrameType {
    fn from(val: u32) -> FrameType {
        match val {
            0 => FrameType::Response,
            1 => FrameType::Error,
            2 => FrameType::Message,
            n => FrameType::Unknown(n),
        }
    }
}


impl<'r, R: std::io::Read> ProtocolReader<'r, R> {
    pub fn new(r: &mut R) -> ProtocolReader<R> {
        let buf = Vec::with_capacity(64);
        ProtocolReader {
            r: r,
            buf: buf,
            // write_cur: Cursor::new(buf),
        }
    }

    // pub fn read_into_buf(&mut self) -> Result<(), std::io::Error> {
    //     let buf = Vec::with_capacity(64);
    //     let buf_len = buf.len();
    //     self.r.read(&mut buf)
    // }

    fn read_into_buf_unless_len_at_least(&mut self, at_least: usize) -> std::io::Result<()> {
        let mut buf_used = self.buf.len();
        println!("read_into_buf_unless_len_at_least({}), buf_used: {}", at_least, buf_used);
        if buf_used >= at_least {
            return Ok(())
        }

        let remaining_to_read = at_least - buf_used;
        self.buf.reserve(remaining_to_read);
        self.buf.extend(std::iter::repeat(0).take(remaining_to_read));
        loop {
            let n = try!(self.r.read(&mut self.buf[buf_used..]));
            buf_used += n;
            if buf_used >= at_least {
                return Ok(());
            }
            if n == 0 {
                // panic!("read_into_buf_unless_len_at_least({}), buf_used: {}", at_least, buf_used);
                return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "zero length read"));
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "buf not full enough after read"))
    }

    pub fn read_frame(&mut self) -> Result<(FrameType, Vec<u8>), std::io::Error> {
        try!(self.read_into_buf_unless_len_at_least(4 + 4)); // frame_length + frame_type
        let mut frame_length = [0u8; 4];
        for (&x, p) in self.buf.iter().zip(frame_length.iter_mut()) {
            *p = x;
        }
        let frame_length = u32::from_be(unsafe { std::mem::transmute(frame_length) }) as usize;

        try!(self.read_into_buf_unless_len_at_least(4 + frame_length as usize));
        let mut frame_type = [0u8; 4];
        for (&x, p) in self.buf[4..8 as usize].iter().zip(frame_type.iter_mut()) {
            *p = x;
        }
        let frame_type = FrameType::from(u32::from_be(unsafe { std::mem::transmute(frame_type) }));
        // let z = frame_length;
        // panic!("foo: {:?}", z);
        let data = self.buf[8..4+frame_length].iter().cloned().collect();
        // println!("self.buf: {:?}", self.buf);
        // println!("self.buf[8..]: {}", String::from_utf8((self.buf[8..4+frame_length]).to_owned()).unwrap());
        for _ in 0..4+frame_length {
            self.buf.remove(0);
        }

        println!("frame_data: {:?}", data);
        Ok((frame_type, data))
    }
}


#[cfg(test)]
mod test {
    use std;
    use super::*;

    #[test]
    fn smoke() {
        let mut b: Vec<u8> = Vec::new();
        b.extend(b"\x00\x00\x00\x06\x00\x00\x00\x00OK");
        b.extend(b"\x00\x00\x00\x06\x00\x00\x00\x00OK");
        b.extend(b"\x00\x00\x00\x25\x00\x00\x00\x02\x13\xf1\xb2\xd4\x35\x47\xd1\x52\x00\x0308a1b6139740c001hmmmmmm".iter());
        b.extend(b"\x00\x00\x00\x25\x00\x00\x00\x02\x13\xf1\xb4\x71\x13\x13\x63\x53\x00\x0108a1b6139740c005hmmmmmm".iter());

        let mut c = std::io::Cursor::new(&b[..]);
        let mut r = ProtocolReader::new(&mut c);

        {
            let (frame_type, data) = r.read_frame().unwrap();
            assert_eq!(frame_type, FrameType::Response);
            assert_eq!(data, b"OK");
        }

        {
            let (frame_type, data) = r.read_frame().unwrap();
            assert_eq!(frame_type, FrameType::Response);
            assert_eq!(data, b"OK");
        }

        {
            let (frame_type, data) = r.read_frame().unwrap();
            assert_eq!(frame_type, FrameType::Message);
            assert_eq!(data.len(), 33);
            assert_eq!(&data[0..8], b"\x13\xf1\xb2\xd4\x35\x47\xd1\x52");
            assert_eq!(&data[8..10], b"\x00\x03");
            assert_eq!(&data[10..26], b"08a1b6139740c001");
            assert_eq!(&data[26..26+7], b"hmmmmmm");
        }


        {
            let (frame_type, data) = r.read_frame().unwrap();
            assert_eq!(frame_type, FrameType::Message);
            assert_eq!(data.len(), 33);
            assert_eq!(&data[0..8], b"\x13\xf1\xb4\x71\x13\x13\x63\x53");
            assert_eq!(&data[8..10], b"\x00\x01");
            assert_eq!(&data[10..26], b"08a1b6139740c005");
            assert_eq!(&data[26..26+7], b"hmmmmmm");
        }
    }
}
