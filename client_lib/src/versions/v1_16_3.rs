use crate::MpClient;


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
    fn login(client: &mut MpClient<Self>, ip: String, port: u16) -> Result<(), String> {
        client.conn.write_packet(Packet753::Handshake { 0: HandshakeSpec {
            version: VarInt(753),
            server_address: ip,
            server_port: port,
            next_state: HandshakeNextState::Login,
        } }).map_err(|e| e.to_string())?;
        client.conn.set_state(State::Login);
        client.conn.write_packet(Packet753::LoginStart { 0: LoginStartSpec {
            name: client.user.login.username.clone(),
        } }).map_err(|e| e.to_string())?;
        // let encryption_packet = conn.read_packet::<RawPacket753>()
        //     .map_err(|e| e.to_string())?
        //     .ok_or("Did not receive encryption packet!".to_string())?;
        // if let Packet753::LoginEncryptionRequest(p) = encryption_packet {
        //     println!("encryption packet: {:?}", p);
        // } else {
        //     return Err("Received the wrong packet!".to_string());
        // }
        let packet_encryption_or_success = client.conn.read_packet::<RawPacket753>()
            .map_err(|e| e.to_string())?
            .ok_or("Did not receive a packet!".to_string())?;
        match packet_encryption_or_success {
            Packet753::LoginEncryptionRequest(p) => {
                println!("encryption packet: {:?}", p);
            },
            Packet753::LoginSuccess(p) => {
                println!("Offline mode server!");
            },
            Packet753::LoginSetCompression(p) => {
                println!("compression packet: {:?}", p);
                client.compression_threshold = *p.threshold;
            },
            _ => return Err(format!("Server sent the wrong packet! Packet sent: {:?}", packet_encryption_or_success)),
        }
        Ok(())
    }
}
