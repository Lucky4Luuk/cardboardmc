#[macro_use] extern crate log;

use glfw::{Action, Context as _, Key, WindowEvent, WindowMode, SwapInterval};
use luminance_glfw::{GlfwSurface, GlfwSurfaceError};

use client_lib::versions::V1_16_3;
use client_lib::MpClient;

mod renderer;

#[derive(Debug)]
pub enum PlatformError {
    CannotCreateWindow,
}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_module("cardboardmc", log::LevelFilter::Debug)
        .filter_module("client_lib", log::LevelFilter::Debug)
        .init();

    debug!("Hello, world!");

    let surface = GlfwSurface::new(|glfw| {
        let (mut window, events) = glfw
            .create_window(1280, 720, "cardboard mc", WindowMode::Windowed)
            .ok_or_else(|| GlfwSurfaceError::UserError(PlatformError::CannotCreateWindow))?;

        window.make_current();
        window.set_all_polling(true);
        glfw.set_swap_interval(SwapInterval::Sync(1));

        Ok((window, events))
    }).expect("GLFW surface creation");

    let mut context = surface.context;
    let events = surface.events_rx;

    let user = client_lib::auth();
    info!("Authenticated as: {}", user.login.username);
    let mut mpc = MpClient::<V1_16_3>::new(user.clone(), "localhost", None);
    mpc.login();

    'game_loop: loop {

    }

    info!("Connected!");
}
