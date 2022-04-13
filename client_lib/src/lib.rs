#[macro_use] extern crate log;

use std::marker::PhantomData;

use craftio_rs::{
    CraftConnection,
};

pub mod versions;
use versions::Version;

mod userdata;
pub use userdata::*;

pub mod assets;
pub mod world;

pub type Connection = CraftConnection<std::io::BufReader<std::net::TcpStream>, std::net::TcpStream>;

pub struct MpClient<V: Version> {
    version: PhantomData<V>,

    user: User,
    conn: Connection,
    ip: String,
    port: u16,

    pub(crate) compression_threshold: i32,
}

impl<V: Version> MpClient<V> {
    pub fn new(user: User, ip: &str, port: Option<u16>) -> Self {
        let port_real = port.unwrap_or(25565);
        let conn = CraftConnection::connect_server_std(&format!("{}:{}", ip, port_real)).expect("Failed to connect to server!"); //TODO: Error handling
        Self {
            version: PhantomData,

            user: user,
            conn: conn,
            ip: ip.to_owned(),
            port: port_real,

            compression_threshold: 0,
        }
    }

    pub fn login(&mut self) {
        let ip = self.ip.clone();
        let port = self.port.clone();
        V::login(self, ip, port).expect("Failed to log in!"); //TODO: Error handling
    }
}
