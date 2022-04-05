use std::marker::PhantomData;

use craftio_rs::{
    CraftConnection,
};

pub mod versions;
use versions::Version;

mod userdata;
pub use userdata::User;

pub type Connection = CraftConnection<std::io::BufReader<std::net::TcpStream>, std::net::TcpStream>;

pub struct MpClient<V: Version> {
    version: PhantomData<V>,

    user: User,
    conn: Connection,
    ip: String,
    port: u16,
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
        }
    }

    pub fn login(&mut self) {
        V::login(&self.user, &mut self.conn, self.ip.clone(), self.port).expect("Failed to log in!"); //TODO: Error handling
    }
}
