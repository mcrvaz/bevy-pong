use super::game_entities::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn spawn_score(mut commands: Commands) {
    commands.spawn().insert(MatchScore {
        score: HashMap::from([(Team::Player, 0), (Team::AI, 0)]),
    });
}

pub fn spawn_paddles(mut commands: Commands) {
    spawn_player_paddle(&mut commands);
    spawn_enemy_paddle(&mut commands);
}

pub fn spawn_ball(mut commands: Commands) {
    commands.spawn_bundle(BallBundle {
        ball: Ball {
            speed_multiplier: 1.05,
        },
        speed: Speed {
            initial: 500.0,
            current: 500.0,
        },
        mov_dir: MovementDirection::from_random_horizontal(),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Option::Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform::default(),
            ..default()
        },
    });
}

pub fn spawn_bounds(window: Res<WindowDescriptor>, mut commands: Commands) {
    let height = window.height;
    let width = window.width;
    let half_height = height / 2.0;
    let half_width = width / 2.0;
    let fixed_size = 25.0;
    spawn_bound(
        &mut commands,
        &Vec2::new(width, fixed_size),
        &Vec3::new(0.0, half_height, 0.0),
        &Quat::IDENTITY,
    );
    spawn_bound(
        &mut commands,
        &Vec2::new(width, fixed_size),
        &Vec3::new(0.0, -half_height, 0.0),
        &Quat::IDENTITY,
    );
    let right_bound = spawn_bound(
        &mut commands,
        &Vec2::new(fixed_size, height),
        &Vec3::new(half_width, 0.0, 0.0),
        &Quat::IDENTITY,
    );
    let left_bound = spawn_bound(
        &mut commands,
        &Vec2::new(fixed_size, height),
        &Vec3::new(-half_width, 0.0, 0.0),
        &Quat::IDENTITY,
    );
    commands
        .entity(left_bound)
        .insert(Goal { team: Team::Player });
    commands.entity(right_bound).insert(Goal { team: Team::AI });
}

fn spawn_player_paddle(mut commands: &mut Commands) {
    let entity = spawn_paddle(&mut commands, &Vec3::new(-500.0, 0.0, 0.0));
    commands.entity(entity).insert(PlayerPaddle);
}

fn spawn_enemy_paddle(mut commands: &mut Commands) {
    let entity = spawn_paddle(&mut commands, &Vec3::new(500.0, 0.0, 0.0));
    commands.entity(entity).insert(AIPaddle);
}

fn spawn_paddle(commands: &mut Commands, translation: &Vec3) -> Entity {
    commands
        .spawn_bundle(PaddleBundle {
            speed: Speed {
                initial: 500.0,
                current: 500.0,
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Option::Some(Vec2::new(50.0, 250.0)),
                    ..default()
                },
                transform: Transform {
                    translation: *translation,
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .id()
}

fn spawn_bound(
    commands: &mut Commands,
    size: &Vec2,
    translation: &Vec3,
    rotation: &Quat,
) -> Entity {
    commands
        .spawn_bundle(BoundsBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Option::Some(*size),
                    ..default()
                },
                transform: Transform {
                    translation: *translation,
                    rotation: *rotation,
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .id()
}
