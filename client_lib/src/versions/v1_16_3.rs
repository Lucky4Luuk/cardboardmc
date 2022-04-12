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
    /// See: https://wiki.vg/Protocol_FAQ#What.27s_the_normal_login_sequence_for_a_client.3F
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
                        client.conn.set_state(State::Play);
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
        if let Packet753::PlayJoinGame(_p) = join_game_packet {
            // println!("join game packet: {:?}", p);
        } else {
            return Err(format!("Server sent the wrong packet! Packet sent: {:?}", join_game_packet));
        }

        let mut expected_packets = 15;
        let mut player_location = None;
        let player_on_ground = true;
        while expected_packets > 0 {
            if let Some(next_packet) = client.conn.read_packet::<RawPacket753>().map_err(|e| e.to_string())? {
                match next_packet {
                    // Optional packets
                    Packet753::PlayServerPluginMessage(_p) => { expected_packets += 1; },
                    Packet753::PlayServerDifficulty(_p) => { expected_packets += 1; },
                    Packet753::PlayServerPlayerAbilities(_p) => { expected_packets += 1; },

                    // Required packets
                    Packet753::PlayServerHeldItemChange(_p) => {},          // 16
                    Packet753::PlayDeclareRecipes(_p) => {},                // 17
                    Packet753::PlayTags(_p) => {},                          // 18
                    Packet753::PlayEntityStatus(_p) => {},                  // 19
                    Packet753::PlayDeclareCommands(_p) => {},               // 20
                    Packet753::PlayUnlockRecipes(_p) => {},                 // 21
                    Packet753::PlayServerPlayerPositionAndLook(p) => {      // 22/30
                        println!("server player pos look: {:?}", p);
                        player_location = Some(p.location);
                        println!("Sending teleport confirm...");
                        client.conn.write_packet(Packet753::PlayTeleportConfirm { 0: PlayTeleportConfirmSpec {
                            teleport_id: p.teleport_id,
                        } }).expect("Failed to write packet to server!");
                    },
                    Packet753::PlayPlayerInfo(_p) => {},                    // 23/24
                    Packet753::PlayUpdateViewPosition(_p) => {},            // 25
                    Packet753::PlayUpdateLight(_p) => {},                   // 26
                    Packet753::PlayChunkData(_p) => {},                     // 27
                    Packet753::PlayWorldBorder(_p) => {},                   // 28
                    Packet753::PlaySpawnPosition(_p) => {},                 // 29
                    _ => return Err(format!("Server sent the wrong packet! Packet sent: {:?}", next_packet)),
                }
                expected_packets -= 1;
            } else {
                break;
            }
        }

        client.conn.write_packet(Packet753::PlayClientPlayerPositionAndRotation { 0: PlayClientPlayerPositionAndRotationSpec {
            feet_location: player_location.unwrap(),
            on_ground: player_on_ground,
        } }).expect("Failed to write packet to server!");

        Ok(())
    }

    fn handle_packet(client: &mut MpClient<Self>) -> bool {
        if let Some(next_packet) = client.conn.read_packet::<RawPacket753>().unwrap() {
            match next_packet {
                Packet753::PlayServerHeldItemChange(_p) => {},
                Packet753::PlayTags(_p) => {},
                Packet753::PlayEntityStatus(_p) => {},
                Packet753::PlayUnlockRecipes(_p) => {},
                Packet753::PlayServerPlayerPositionAndLook(_p) => {},
                Packet753::PlayPlayerInfo(_p) => {},
                Packet753::PlayChunkData(_p) => {},
                Packet753::PlayWorldBorder(_p) => {},
                Packet753::PlaySpawnPosition(_p) => {},
                Packet753::PlayUpdateViewPosition(_p) => {},
                Packet753::PlayUpdateLight(_p) => {},
                Packet753::PlaySpawnLivingEntity(_p) => {},
                Packet753::PlayEntityMetadata(_p) => {},
                Packet753::PlayEntityProperties(_p) => {},
                Packet753::PlayEntityEquipment(_p) => {},
                Packet753::PlaySpawnEntity(_p) => {},
                Packet753::PlayEntityVelocity(_p) => {},
                Packet753::PlayTimeUpdate(_p) => {},
                Packet753::PlayWindowItems(_p) => {},
                Packet753::PlaySetSlot(_p) => {},
                _ => panic!("Server sent the wrong packet! Packet sent: {:?}", next_packet),
            }
            true
        } else {
            false
        }
    }
}
