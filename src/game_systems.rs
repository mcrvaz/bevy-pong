use crate::utils::{approx_eq, inverse_lerp, lerp, random_horizontal, rotate_vec2};

use super::{
    game_entities::*, input,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

pub fn ball_launch_timer(
    time: Res<Time>,
    mut launch_ev: EventReader<BallLaunch>,
    mut timer: ResMut<BallLaunchDelay>,
) {
    for _ in launch_ev.iter() {
        timer.0.reset();
    }

    timer.0.tick(time.delta());
}

pub fn is_ball_launch_ready(timer: Res<BallLaunchDelay>) -> bool {
    timer.0.just_finished()
}

pub fn was_ball_launched(timer: Res<BallLaunchDelay>) -> bool {
    timer.0.finished()
}

pub fn start_ball_movement(mut query: Query<(&Ball, &mut Velocity, &mut Transform)>) {
    for (ball, mut velocity, mut transform) in query.iter_mut() {
        set_initial_ball_position(&mut transform);
        launch_ball(ball, &mut velocity);
    }
}

pub fn paddle_movement(
    axis_inputs: Query<&input::InputAxes>,
    mut query: Query<(&mut Velocity, &Paddle), With<PlayerPaddle>>,
) {
    let input = axis_inputs.single();
    let vertical_input = input.val.get(&input::Axis::Vertical).unwrap();
    for (mut rb, paddle) in query.iter_mut() {
        rb.linvel.y = vertical_input.val * paddle.speed;
    }
}

pub fn evaluate_ball_collision(
    mut ev_goal: EventWriter<GoalEvent>,
    mut ball_query: Query<(Entity, &Transform, &mut Velocity, &Ball)>,
    paddle_query: Query<(Entity, &Transform, &Collider), With<Paddle>>,
    goal_query: Query<(Entity, &Goal)>,
    rapier_context: Res<RapierContext>,
) {
    for (b_entity, b_transform, mut b_velocity, b) in ball_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(b_entity) {
            let other = if b_entity == contact_pair.collider1() {
                contact_pair.collider2()
            } else {
                contact_pair.collider1()
            };

            let opt_goal = || goal_query.iter().find(|x| x.0 == other);
            let opt_paddle = paddle_query.iter().find(|x| x.0 == other);
            if let Some((_, p_transform, p_collider)) = opt_paddle {
                handle_ball_paddle_collision(
                    p_transform,
                    p_collider,
                    b_transform,
                    &mut b_velocity,
                    b,
                );
            } else if let Some((_, goal)) = opt_goal() {
                handle_ball_goal_collision(&mut ev_goal, goal, b_entity.id());
            }
        }
    }
}

pub fn handle_ball_paddle_collision(
    p_transform: &Transform,
    p_collider: &Collider,
    b_transform: &Transform,
    b_velocity: &mut Velocity,
    b: &Ball,
) {
    let col_extents = p_collider.as_cuboid().unwrap().half_extents();

    let p_min = p_transform.translation.truncate() - col_extents;
    let p_max = p_transform.translation.truncate() + col_extents;
    let b_position = b_transform.translation.truncate();

    const MIN_ANGLE: f32 = 45.0_f32;
    const MAX_ANGLE: f32 = -45.0_f32;
    let reflection_ratio = inverse_lerp(p_min.y, p_max.y, b_position.y);
    let mut reflection_radians = lerp(MIN_ANGLE, MAX_ANGLE, reflection_ratio).to_radians();
    if reflection_radians != 0.0 {
        // a bit of noise to prevent the ball from always hitting the same spot
        reflection_radians += random::<f32>().to_radians();
    }

    b_velocity.linvel = rotate_vec2(b_velocity.linvel * b.speed_multiplier, reflection_radians);
}

pub fn handle_ball_goal_collision(ev_goal: &mut EventWriter<GoalEvent>, goal: &Goal, ball_id: u32) {
    ev_goal.send(GoalEvent {
        team: goal.team.opposite(),
        ball_id: ball_id,
    });
}

pub fn score(mut ev_goal: EventReader<GoalEvent>, mut query: Query<&mut MatchScore>) {
    for ev in ev_goal.iter() {
        let mut match_score = query.single_mut();
        let team_score = match_score.score.get_mut(&ev.team).unwrap();
        *team_score += 1;
    }
}

pub fn reset_ball(
    mut launch_ev: EventWriter<BallLaunch>,
    mut ev_goal: EventReader<GoalEvent>,
    mut ball_query: Query<(Entity, &mut Velocity, &mut Transform)>,
) {
    for ev in ev_goal.iter() {
        let (_, mut velocity, mut transform) = ball_query
            .iter_mut()
            .find(|x| x.0.id() == ev.ball_id)
            .unwrap();
        set_initial_ball_position(&mut transform);
        set_initial_ball_speed(&mut velocity);
        launch_ev.send(BallLaunch);
    }
}

pub fn set_initial_ball_position(mut transform: &mut Transform) {
    transform.translation = Vec3::ZERO;
    transform.rotation = Quat::IDENTITY;
}

pub fn set_initial_ball_speed(mut velocity: &mut Velocity) {
    velocity.linvel = Vec2::ZERO;
    velocity.angvel = 0.0;
}

pub fn launch_ball(ball: &Ball, mut velocity: &mut Velocity) {
    velocity.linvel = random_horizontal() * ball.initial_speed;
}

pub fn prevent_stuck_ball(mut query: Query<&mut Velocity, With<Ball>>) {
    const MIN_V: f32 = 100.0;
    for mut v in query.iter_mut() {
        if approx_eq(v.linvel.x, 0.0, MIN_V) {
            v.linvel.x += MIN_V * v.linvel.x.signum();
        }
    }
}

pub fn limit_ball_velocity(mut query: Query<(&mut Velocity, &Ball)>) {
    for (mut v, ball) in query.iter_mut() {
        v.linvel = Vec2::min(v.linvel, ball.max_speed);
    }
}

pub fn enemy_paddle_movement(
    ball_query: Query<(Entity, &Transform), With<Ball>>,
    mut paddle_query: Query<(&Paddle, &AIPaddle, &Transform, &mut Velocity)>,
) {
    let (paddle, ai_paddle, paddle_transform, mut paddle_vel) = paddle_query.single_mut();
    let (_, ball_transform) = ball_query
        .iter()
        .find(|x| x.0.id() == ai_paddle.target_ball)
        .or(ball_query.iter().next())
        .unwrap();

    let y_diff = ball_transform.translation.y - paddle_transform.translation.y;
    const MAX_DELTA: f32 = 45.0_f32;
    if approx_eq(y_diff, 0.0, MAX_DELTA) {
        paddle_vel.linvel.y = 0.0;
    } else {
        paddle_vel.linvel.y = y_diff.signum() * paddle.speed;
    }
}
