use super::{game_entities::*, input, utils};

use bevy::{prelude::*, sprite::collide_aabb};
use rand::prelude::*;

pub struct PongGame;
impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(setup_camera)
            .add_startup_system(spawn_ball)
            .add_startup_system(spawn_paddles)
            .add_startup_system(spawn_bounds)
            .add_system_set(
                SystemSet::new()
                    .label(Label::Default)
                    .after(input::Label::Default)
                    .with_system(paddle_movement)
                    .with_system(ball_collision)
                    .with_system(ball_movement),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Default,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_paddles(mut commands: Commands) {
    spawn_player_paddle(&mut commands);
    spawn_enemy_paddle(&mut commands);
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
            speed: Speed(500.0),
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

fn spawn_ball(mut commands: Commands) {
    commands.spawn_bundle(BallBundle {
        ball: Ball {
            speed_multiplier: 1.05,
        },
        speed: Speed(500.0),
        mov_dir: MovementDirection(Vec3::new(random::<f32>(), 0.0, 0.0).normalize_or_zero()),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Option::Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform::default(),
            ..default()
        },
        ..default()
    });
}

fn spawn_bounds(mut commands: Commands) {
    let height = 1080.0;
    let width = 1920.0;
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
    spawn_bound(
        &mut commands,
        &Vec2::new(fixed_size, height),
        &Vec3::new(half_width, 0.0, 0.0),
        &Quat::IDENTITY,
    );
    spawn_bound(
        &mut commands,
        &Vec2::new(fixed_size, height),
        &Vec3::new(-half_width, 0.0, 0.0),
        &Quat::IDENTITY,
    );
}

fn spawn_bound(commands: &mut Commands, size: &Vec2, translation: &Vec3, rotation: &Quat) {
    commands.spawn_bundle(BoundsBundle {
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
    });
}

fn paddle_movement(
    time: Res<Time>,
    axis_inputs: Query<&input::InputAxes>,
    mut query: Query<
        (&mut Transform, &mut MovementDirection, &Speed),
        (With<Paddle>, With<PlayerPaddle>),
    >,
) {
    let input = axis_inputs.single();
    let vertical_input = input.val.get(&input::Axis::Vertical).unwrap();
    for (mut transform, mut mov_dir, speed) in query.iter_mut() {
        let previous_pos = transform.translation;
        transform.translation.y += vertical_input.val * speed.0 * time.delta_seconds();
        mov_dir.0 = (transform.translation - previous_pos).normalize_or_zero();
    }
}

fn ball_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MovementDirection, &Speed), With<Ball>>,
) {
    for (mut transform, move_dir, speed) in query.iter_mut() {
        transform.translation += move_dir.0 * speed.0 * time.delta_seconds();
    }
}

fn ball_collision(
    mut ball_query: Query<(
        &Sprite,
        &Transform,
        &Ball,
        &mut MovementDirection,
        &mut Speed,
    )>,
    colliders_query: Query<(&Sprite, &Transform, Option<&MovementDirection>), Without<Ball>>,
) {
    for (ball_spr, ball_transform, ball, mut ball_mov_dir, mut ball_speed) in ball_query.iter_mut()
    {
        for (coll_spr, coll_transform, coll_mov_dir) in colliders_query.iter() {
            let collision = collide_aabb::collide(
                ball_transform.translation,
                ball_spr.custom_size.unwrap(),
                coll_transform.translation,
                coll_spr.custom_size.unwrap(),
            );
            if let Some(c) = collision {
                let collision_dir = utils::v2_to_v3(utils::collision_to_direction(c) * 2.0);
                let mut vectors = vec![collision_dir, ball_mov_dir.0];

                if let Some(c) = coll_mov_dir {
                    vectors.push(c.0);
                }
                let result_dir = utils::avg(&vectors);
                ball_mov_dir.0 = result_dir.normalize_or_zero();
                ball_speed.0 *= ball.speed_multiplier;
            }
        }
    }
}
