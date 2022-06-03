use bevy::prelude::*;
mod game;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong!".to_string(),
            width: 1920.0,
            height: 1080.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(game::PongGame)
        .run();
}
