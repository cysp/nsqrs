// #![feature(drain)]

// extern crate byteorder;
extern crate serde;


mod error;
mod result;

mod connection;
mod client;

pub use error::Error;
pub use result::Result;

pub mod protocol;

pub use protocol::identification::{Identification, IdentificationBuilder};
pub use protocol::message::MessageId;

pub use connection::Connection;
pub use client::Client;

pub use connection::{Frame, ResponseFrame, ErrorFrame};


pub fn connect(addr: std::net::SocketAddr) -> Result<Connection> {
    Connection::connect(addr)
}
