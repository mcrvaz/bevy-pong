use bevy::{prelude::*, window::PresentMode};
mod game;
mod game_systems;
mod game_entities;
mod game_setup_systems;
mod game_ui_systems;
mod game_ui_setup_systems;
mod input;
mod utils;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong!".to_string(),
            width: 1920.0,
            height: 1080.0,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(bevy_framepace::FramepacePlugin {
            framerate_limit: bevy_framepace::FramerateLimit::Manual(144),
            warn_on_frame_drop: false,
        })
        .add_plugin(input::PongInput)
        .add_plugin(game::PongGame)
        .run();
}
