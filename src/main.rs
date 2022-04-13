#[macro_use] extern crate log;

use client_lib::versions::V1_16_3;
use client_lib::MpClient;

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_module("cardboardmc", log::LevelFilter::Debug)
        .filter_module("client_lib", log::LevelFilter::Debug)
        .init();

    debug!("Hello, world!");

    let user = client_lib::auth();
    info!("Authenticated as: {}", user.login.username);
    let mut mpc = MpClient::<V1_16_3>::new(user.clone(), "localhost", None);
    mpc.login();

    info!("Connected!");
}
