use super::input;
use bevy::{
    prelude::*,
    sprite::{self, collide_aabb},
};
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
                    .with_system(ball_movement)
                    .with_system(paddle_ball_collision),
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
struct Paddle;

#[derive(Component)]
struct PlayerPaddle;

#[derive(Component)]
struct AIPaddle;

#[derive(Component)]
struct Ball {
    movement_dir: Vec3,
}

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
        .insert(Speed { val: 10.0 })
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
        .insert(Ball {
            // movement_dir: Vec3::new(random::<f32>(), random::<f32>(), 0.0),
            movement_dir: Vec3::new(random::<f32>(), 0.0, 0.0),
        })
        .insert(Speed { val: 10.0 });
}

fn paddle_movement(
    axis_inputs: Query<&input::InputAxes>,
    mut query: Query<(&mut Transform, &Speed), (With<Paddle>, With<PlayerPaddle>)>,
) {
    let input = axis_inputs.single();
    let vertical_input = input.val.get(&input::Axis::Vertical).unwrap();
    for (mut transform, speed) in query.iter_mut() {
        transform.translation.y += vertical_input.val * speed.val;
    }
}

fn ball_movement(mut query: Query<(&mut Transform, &Ball, &Speed)>) {
    for (mut transform, ball, speed) in query.iter_mut() {
        transform.translation += ball.movement_dir * speed.val;
    }
}

fn paddle_ball_collision(
    ball_query: Query<(&Sprite, &Transform), With<Ball>>,
    paddle_query: Query<(&Sprite, &Transform), With<Paddle>>,
) {
    for (ball_spr, ball_transform) in ball_query.iter() {
        for (paddle_spr, paddle_transform) in paddle_query.iter() {
            let collision = collide_aabb::collide(
                ball_transform.translation,
                ball_spr.custom_size.unwrap(),
                paddle_transform.translation,
                paddle_spr.custom_size.unwrap(),
            );
            if collision.is_some() {
                println!("Collided!");
            }
            let direction = collision_to_direction(collision);
        }
    }
}

fn collision_to_direction(collision: Option<collide_aabb::Collision>) -> Vec2 {
    match collision {
        Some(dir) => match dir {
            collide_aabb::Collision::Left => Vec2::new(-1.0, 0.0),
            collide_aabb::Collision::Right => Vec2::new(1.0, 0.0),
            collide_aabb::Collision::Top => Vec2::new(0.0, 1.0),
            collide_aabb::Collision::Bottom => Vec2::new(0.0, -1.0),
            collide_aabb::Collision::Inside => Vec2::ZERO,
        },
        None => Vec2::ZERO,
    }
}
