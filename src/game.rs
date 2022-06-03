use bevy::prelude::*;

pub struct PongGame;

#[derive(Component)]
struct Paddle;
#[derive(Component)]
struct PlayerPaddle;

impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(setup_camera)
            .add_startup_system(spawn_player_paddle)
            .add_startup_system(spawn_enemy_paddle)
            .add_system(paddle_movement);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player_paddle(mut commands: Commands) {
    let paddle_entity = spawn_paddle(&mut commands, &Vec3::new(-500.0, 0.0, 0.0));
    commands.entity(paddle_entity).insert(PlayerPaddle);
}

fn spawn_enemy_paddle(mut commands: Commands) {
    spawn_paddle(&mut commands, &Vec3::new(500.0, 0.0, 0.0));
}

fn spawn_paddle(commands: &mut Commands, translation: &Vec3) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: *translation,
                scale: Vec3::new(50.0, 250.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Paddle)
        .id()
}

fn paddle_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut positions: Query<&mut Transform, With<PlayerPaddle>>,
) {
    for mut transform in positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.0;
        }
    }
}
