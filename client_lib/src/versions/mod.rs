use super::Connection;

mod protocols;

mod v1_16_3;
pub use v1_16_3::V1_16_3;

pub trait Version {
    fn login(conn: &mut Connection, ip: String, port: u16) -> Result<(), String>;
}