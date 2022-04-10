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

        // Attempt to log into server
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
                    Packet753::LoginSuccess(_p) => {
                        println!("Login success!");
                        break;
                    },
                    Packet753::LoginSetCompression(p) => {
                        println!("compression packet: {:?}", p);
                        client.compression_threshold = *p.threshold;
                        client.conn.set_compression_threshold(Some(*p.threshold));
                        expected_packets += 1; // We expect the next packet to be a LoginSuccess packet
                    },
                    _ => return Err(format!("Server sent the wrong packet! Packet sent: {:?}", next_packet)),
                }
            } else {
                break;
            }
        }

        println!("Logged into server...");

        // We should now be succesfully logged into the server
        // and ready to accept player info and such
        let join_game_packet = client.conn.read_packet::<RawPacket753>().map_err(|e| e.to_string())?.ok_or("Expected a Join game packet, did not receive any!")?;
        if let Packet753::PlayJoinGame(p) = join_game_packet {
            println!("join game packet: {:?}", p);
        }

        Ok(())
    }
}
