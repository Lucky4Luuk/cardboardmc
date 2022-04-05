use crate::Connection;

use mcproto_rs::types::VarInt;
use super::protocols::v1_16_3::*;

use craftio_rs::CraftSyncWriter;

pub struct V1_16_3 {

}

impl super::Version for V1_16_3 {
    fn login(conn: &mut Connection, ip: String, port: u16) -> Result<(), String> {
        conn.write_packet(Packet753::Handshake { 0: HandshakeSpec {
            version: VarInt(753),
            server_address: ip,
            server_port: port,
            next_state: HandshakeNextState::Login,
        } }).map_err(|e| e.to_string())?;
        Ok(())
    }
}
