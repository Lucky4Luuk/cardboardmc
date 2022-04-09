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
        let mut expected_packets = 1;
        while expected_packets > 0 {
            expected_packets -= 1;
            println!("Waiting for packet...");
            if let Some(next_packet) = client.conn.read_packet::<RawPacket753>().map_err(|e| e.to_string())? {
                match next_packet {
                    Packet753::LoginEncryptionRequest(p) => {
                        println!("encryption packet: {:?}", p);
                        todo!("Implement encryption for login to online mode servers");
                    },
                    Packet753::LoginSuccess(p) => {
                        println!("Offline mode server!");
                        break;
                    },
                    Packet753::LoginSetCompression(p) => {
                        println!("compression packet: {:?}", p);
                        client.compression_threshold = *p.threshold;
                        expected_packets += 1; // We expect the next packet to be a LoginSuccess packet
                    },
                    _ => return Err(format!("Server sent the wrong packet! Packet sent: {:?}", next_packet)),
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}
