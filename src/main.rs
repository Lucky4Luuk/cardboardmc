use client_lib::versions::V1_16_3;
use client_lib::{
    MpClient,
    User,
};

fn main() {
    println!("Hello, world!");

    let user = client_lib::auth();
    println!("Authenticated as: {}", user.login.username);
    let mut mpc = MpClient::<V1_16_3>::new(user.clone(), "localhost", None);
    mpc.login();

    println!("Logged in!");
}
