use crate::MpClient;

mod protocols;

mod v1_16_3;
pub use v1_16_3::V1_16_3;

pub trait Version {
    fn login(client: &mut MpClient<Self>, ip: String, port: u16) -> Result<(), String> where Self: std::marker::Sized;
}
