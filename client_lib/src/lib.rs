use std::marker::PhantomData;

use craftio_rs::{
    CraftConnection,
    CraftIo,
};
use mcproto_rs::protocol::State;

pub mod versions;
use versions::Version;

pub type Connection = CraftConnection<std::io::BufReader<std::net::TcpStream>, std::net::TcpStream>;

pub struct MpClient<V: Version> {
    version: PhantomData<V>,

    conn: Connection,
    ip: String,
    port: u16,
}

impl<V: Version> MpClient<V> {
    pub fn new(ip: &str, port: Option<u16>) -> Self {
        let port_real = port.unwrap_or(25565);
        let conn = CraftConnection::connect_server_std(&format!("{}:{}", ip, port_real)).expect("Failed to connect to server!"); //TODO: Error handling
        Self {
            version: PhantomData,

            conn: conn,
            ip: ip.to_owned(),
            port: port_real,
        }
    }

    pub fn login(&mut self) {
        V::login(&mut self.conn, self.ip.clone(), self.port).expect("Failed to log in!"); //TODO: Error handling
        self.conn.set_state(State::Login);
    }
}
