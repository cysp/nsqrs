extern crate nsqrs;


fn main() {
    let mut b: Vec<u8> = Vec::new();
    b.extend(b"\x00\x00\x01\x0c\x00\x00\x00\x00{\"max_rdy_count\":2500,\"version\":\"0.3.5\",\"max_msg_timeout\":900000,\"msg_timeout\":60000,\"tls_v1\":false,\"deflate\":false,\"deflate_level\":0,\"max_deflate_level\":6,\"snappy\":false,\"sample_rate\":0,\"auth_required\":false,\"output_buffer_size\":16384,\"output_buffer_timeout\":250}".iter());
    b.extend(b"\x00\x00\x00\x06\x00\x00\x00\x00OK");
    b.extend(b"\x00\x00\x00\x06\x00\x00\x00\x00OK");
    b.extend(b"\x00\x00\x00\x25\x00\x00\x00\x02\x13\xf1\xb2\xd4\x35\x47\xd1\x52\x00\x0308a1b6139740c001hmmmmmm".iter());
    b.extend(b"\x00\x00\x00\x25\x00\x00\x00\x02\x13\xf1\xb4\x71\x13\x13\x63\x53\x00\x0108a1b6139740c005hmmmmmm".iter());

    let mut c = std::io::Cursor::new(&b[..]);
    let mut r = nsqrs::protocol::reading::ProtocolReader::new(&mut c);


    loop {
        match r.read_frame() {
            Ok(f) => println!("frame: {:?}", f),
            Err(e) => {
                println!("error: {:?}", e);
                return;
            }
        }
    }
}
