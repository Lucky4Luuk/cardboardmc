use crate::{
    Connection,
    User,
};


use mcproto_rs::protocol::State;
use mcproto_rs::types::VarInt;
use super::protocols::v1_16_3::*;

use craftio_rs::{
    CraftSyncWriter,
    CraftSyncReader,
    CraftIo,
};

pub struct V1_16_3 {

}

impl super::Version for V1_16_3 {
    fn login(user: &User, conn: &mut Connection, ip: String, port: u16) -> Result<(), String> {
        conn.write_packet(Packet753::Handshake { 0: HandshakeSpec {
            version: VarInt(753),
            server_address: ip,
            server_port: port,
            next_state: HandshakeNextState::Login,
        } }).map_err(|e| e.to_string())?;
        conn.set_state(State::Login);
        conn.write_packet(Packet753::LoginStart { 0: LoginStartSpec {
            name: user.login.username.clone(),
        } }).map_err(|e| e.to_string())?;
        let encryption_packet = conn.read_packet::<RawPacket753>()
            .map_err(|e| e.to_string())?
            .ok_or("Did not receive encryption packet!".to_string())?;
        if let Packet753::LoginEncryptionRequest(p) = encryption_packet {
            // println!("encryption packet: {:?}", p);
        } else {
            return Err("Received the wrong packet!".to_string());
        }
        Ok(())
    }
}
