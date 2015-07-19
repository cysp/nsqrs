extern crate nsqrs;


fn main() {
    let mut nsq_conn = nsqrs::connect("127.0.0.1:4150".parse().unwrap()).unwrap();
    let id = nsqrs::Identification::builder()
        .client_id("nsqrs-example-bar")
        .hostname("naïve")
        .build();
    nsq_conn.send_identify(id).unwrap();

    nsq_conn.send_sub("foo", "bar").unwrap();

    nsq_conn.send_rdy(4).unwrap();

    let mut num_heartbeats = 0u32;
    let mut num_messages = 0u32;
    loop {
        match nsq_conn.recv_frame() {
            Ok(nsqrs::Frame::ResponseFrame(nsqrs::ResponseFrame::Heartbeat)) => {
                num_heartbeats = num_heartbeats.saturating_add(1);
                println!("heartbeat: {}", num_heartbeats);
                nsq_conn.send_nop().unwrap();
            }
            Ok(nsqrs::Frame::ResponseFrame(nsqrs::ResponseFrame::CloseWait)) => {
                return;
            }
            Ok(nsqrs::Frame::MessageFrame(message_frame)) => {
                num_messages = num_messages.saturating_add(1);
                println!("message: {} {:#?}", num_messages, message_frame);
                nsq_conn.send_fin(message_frame.message_id).unwrap();
                if num_messages == 3 {
                    nsq_conn.send_cls().unwrap();
                }
            }
            Ok(f) => println!("frame: {:?}", f),
            Err(e) => {
                println!("error: {:?}", e);
                return;
            }
        }
    }
}
