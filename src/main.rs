use client_lib::versions::V1_16_3;
use client_lib::MpClient;

fn main() {
    println!("Hello, world!");

    let mut mpc = MpClient::<V1_16_3>::new("localhost", None);
    mpc.login();

    println!("Handshake!");
}
