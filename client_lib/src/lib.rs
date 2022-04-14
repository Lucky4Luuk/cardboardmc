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

use world::World;
use world::chunk::{Chunk, ChunkPos};

pub type Connection = CraftConnection<std::io::BufReader<std::net::TcpStream>, std::net::TcpStream>;

pub struct MpClient<V: Version> {
    version: PhantomData<V>,

    user: User,
    conn: Connection,
    ip: String,
    port: u16,

    pub(crate) compression_threshold: i32,

    pub world: World,
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

            world: World::empty(),
        }
    }

    pub fn login(&mut self) {
        let ip = self.ip.clone();
        let port = self.port.clone();
        V::login(self, ip, port).expect("Failed to log in!"); //TODO: Error handling
    }

    /// Attempts to retrieve a loaded chunk. If the chunk is not loaded, but does exist, it will
    /// be loaded into memory. If a chunk doesn't exist at all, it will create a new, empty chunk.
    //TODO: Load chunk from disk
    pub fn get_chunk(&mut self, chunk_pos: ChunkPos) -> &Chunk {
        if !self.world.has_chunk(chunk_pos) {
            self.world.new_chunk(chunk_pos);
        }
        self.world.get_chunk(chunk_pos).unwrap()
    }

    /// The same as get_chunk, but returns a mutable reference instead.
    pub fn get_chunk_mut(&mut self, chunk_pos: ChunkPos) -> &mut Chunk {
        if !self.world.has_chunk(chunk_pos) {
            self.world.new_chunk(chunk_pos);
        }
        self.world.get_chunk_mut(chunk_pos).unwrap()
    }
}
