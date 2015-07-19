#![allow(dead_code)]

use std;

use result::Result;


pub struct Client {
    addrs: Vec<std::net::SocketAddr>,
    conns: Vec<std::net::TcpStream>,
    pending_conns: Vec<std::net::TcpStream>,
}

impl Client {
    pub fn connect<A: std::net::ToSocketAddrs>(addrs: A) -> Result<Client> {
        Ok(Client {
            addrs: addrs.to_socket_addrs().unwrap().collect(),
            conns: Vec::new(),
            pending_conns: Vec::new(),
        })
    }
}
