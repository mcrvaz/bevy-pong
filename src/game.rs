use super::{input, utils};

use bevy::{prelude::*, sprite::collide_aabb};
use rand::prelude::*;

pub struct PongGame;
impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(setup_camera)
            .add_startup_system(spawn_ball)
            .add_startup_system(spawn_player_paddle)
            .add_startup_system(spawn_enemy_paddle)
            .add_system_set(
                SystemSet::new()
                    .label(Label::Default)
                    .after(input::Label::Default)
                    .with_system(paddle_movement)
                    .with_system(paddle_ball_collision.before(ball_movement))
                    .with_system(ball_movement),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Default,
}

#[derive(Component)]
struct Speed {
    val: f32,
}
#[derive(Component)]
struct MovementDirection {
    val: Vec3,
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct PlayerPaddle;

#[derive(Component)]
struct AIPaddle;

#[derive(Component)]
struct Ball;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player_paddle(mut commands: Commands) {
    let entity = spawn_paddle(&mut commands, &Vec3::new(-500.0, 0.0, 0.0));
    commands.entity(entity).insert(PlayerPaddle);
}

fn spawn_enemy_paddle(mut commands: Commands) {
    let entity = spawn_paddle(&mut commands, &Vec3::new(500.0, 0.0, 0.0));
    commands.entity(entity).insert(AIPaddle);
}

fn spawn_paddle(commands: &mut Commands, translation: &Vec3) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
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
        })
        .insert(Paddle)
        .insert(MovementDirection { val: Vec3::ZERO })
        .insert(Speed { val: 500.0 })
        .id()
}

fn spawn_ball(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Option::Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform { ..default() },
            ..default()
        })
        .insert(Ball)
        .insert(MovementDirection {
            val: Vec3::new(random::<f32>(), 0.0, 0.0).normalize_or_zero(),
        })
        .insert(Speed { val: 500.0 });
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
        transform.translation.y += vertical_input.val * speed.val * time.delta_seconds();
        mov_dir.val = (transform.translation - previous_pos).normalize_or_zero();
    }
}

fn ball_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MovementDirection, &Speed), With<Ball>>,
) {
    for (mut transform, move_dir, speed) in query.iter_mut() {
        transform.translation += move_dir.val * speed.val * time.delta_seconds();
    }
}

fn paddle_ball_collision(
    mut ball_query: Query<
        (&Sprite, &Transform, &mut MovementDirection),
        (With<Ball>, Without<Paddle>),
    >,
    paddle_query: Query<(&Sprite, &Transform, &MovementDirection), (With<Paddle>, Without<Ball>)>,
) {
    for (ball_spr, ball_transform, mut ball_mov_dir) in ball_query.iter_mut() {
        for (paddle_spr, paddle_transform, paddle_mov_dir) in paddle_query.iter() {
            let collision = collide_aabb::collide(
                ball_transform.translation,
                ball_spr.custom_size.unwrap(),
                paddle_transform.translation,
                paddle_spr.custom_size.unwrap(),
            );
            if let Some(c) = collision {
                let collision_dir = utils::v2_to_v3(utils::collision_to_direction(c) * 2.0);
                let result_dir = (collision_dir + paddle_mov_dir.val + ball_mov_dir.val) / 3.0;
                ball_mov_dir.val = result_dir.normalize_or_zero();
            }
        }
    }
}
