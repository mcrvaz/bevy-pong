use super::{
    game_entities::*, game_setup_systems::*, game_ui_setup_systems::*, game_ui_systems::*, input,
    utils,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

pub struct PongGame;
impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(BallLaunchDelay(Timer::from_seconds(0.5, false)))
            .add_event::<BallLaunch>()
            .add_event::<GoalEvent>()
            .add_startup_system_set(
                SystemSet::new()
                    .label(Label::Setup)
                    .with_system(setup_cameras)
                    .with_system(setup_physics)
                    .with_system(spawn_ball)
                    .with_system(spawn_paddles)
                    .with_system(spawn_bounds)
                    .with_system(spawn_score)
                    .with_system(spawn_score_text)
                    .with_system(spawn_ball_launch_timer_text),
            )
            .add_startup_system_set_to_stage(
                StartupStage::PostStartup,
                SystemSet::new().with_system(initial_score),
            )
            .add_system_set(
                SystemSet::new()
                    .label(Label::CollisionCheck)
                    .after(input::Label::Default)
                    .with_system(evaluate_ball_collision),
            )
            .add_system_set(
                SystemSet::new()
                    .label(Label::BallLaunch)
                    .after(Label::CollisionCheck)
                    .with_system(ball_launch_timer)
                    .with_system(update_ball_launch_timer),
            )
            .add_system_set(
                SystemSet::new()
                    .label(Label::Default)
                    .after(Label::BallLaunch)
                    .with_system(start_ball_movement.run_if(is_ball_launch_ready))
                    .with_system(prevent_stuck_ball.run_if(is_ball_moving))
                    .with_system(score)
                    .with_system(reset_ball)
                    .with_system(paddle_movement)
                    .with_system(limit_ball_velocity),
            )
            .add_system_set(
                SystemSet::new()
                    .label(Label::UI)
                    .after(Label::Default)
                    .with_system(update_score_runtime),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Setup,
    CollisionCheck,
    BallLaunch,
    Default,
    UI,
}

fn ball_launch_timer(
    time: Res<Time>,
    mut launch_ev: EventReader<BallLaunch>,
    mut timer: ResMut<BallLaunchDelay>,
) {
    for _ in launch_ev.iter() {
        timer.0.reset();
    }

    timer.0.tick(time.delta());
}

fn is_ball_launch_ready(timer: Res<BallLaunchDelay>) -> bool {
    timer.0.just_finished()
}

fn is_ball_moving(timer: Res<BallLaunchDelay>) -> bool {
    timer.0.finished()
}

fn start_ball_movement(mut query: Query<(&Ball, &mut Velocity, &mut Transform)>) {
    for (ball, mut velocity, mut transform) in query.iter_mut() {
        set_initial_ball_position(&mut transform);
        launch_ball(ball, &mut velocity);
    }
}

fn paddle_movement(
    axis_inputs: Query<&input::InputAxes>,
    mut query: Query<(&mut Velocity, &Paddle), With<PlayerPaddle>>,
) {
    let input = axis_inputs.single();
    let vertical_input = input.val.get(&input::Axis::Vertical).unwrap();
    for (mut rb, paddle) in query.iter_mut() {
        rb.linvel.y = vertical_input.val * paddle.speed;
    }
}

fn evaluate_ball_collision(
    mut ev_goal: EventWriter<GoalEvent>,
    mut ball_query: Query<(Entity, &mut Velocity, &Ball)>,
    paddle_query: Query<Entity, With<Paddle>>,
    goal_query: Query<(Entity, &Goal)>,
    rapier_context: Res<RapierContext>,
) {
    for (ball_entity, mut ball_velocity, ball) in ball_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(ball_entity) {
            let other = if ball_entity == contact_pair.collider1() {
                contact_pair.collider2()
            } else {
                contact_pair.collider1()
            };

            let opt_goal = || goal_query.iter().find(|x| x.0 == other);
            let is_paddle = || paddle_query.contains(other);
            if is_paddle() {
                handle_ball_paddle_collision(&mut ball_velocity, ball);
            } else if let Some((_, goal)) = opt_goal() {
                handle_ball_goal_collision(&mut ev_goal, goal, ball_entity.id());
            }
        }
    }
}

fn handle_ball_paddle_collision(velocity: &mut Velocity, ball: &Ball) {
    velocity.linvel *= ball.speed_multiplier;
}

fn handle_ball_goal_collision(ev_goal: &mut EventWriter<GoalEvent>, goal: &Goal, ball_id: u32) {
    ev_goal.send(GoalEvent {
        team: goal.team.opposite(),
        ball_id: ball_id,
    });
}

fn score(mut ev_goal: EventReader<GoalEvent>, mut query: Query<&mut MatchScore>) {
    for ev in ev_goal.iter() {
        let mut match_score = query.single_mut();
        let team_score = match_score.score.get_mut(&ev.team).unwrap();
        *team_score += 1;
    }
}

fn reset_ball(
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

fn set_initial_ball_position(mut transform: &mut Transform) {
    transform.translation = Vec3::ZERO;
    transform.rotation = Quat::IDENTITY;
}

fn set_initial_ball_speed(mut velocity: &mut Velocity) {
    velocity.linvel = Vec2::ZERO;
    velocity.angvel = 0.0;
}

fn launch_ball(ball: &Ball, mut velocity: &mut Velocity) {
    velocity.linvel = utils::random_horizontal() * ball.initial_speed;
}

fn prevent_stuck_ball(mut query: Query<&mut Velocity, With<Ball>>) {
    const MIN_V: f32 = 25.0;
    for mut v in query.iter_mut() {
        if utils::approx_eq(v.linvel.x, 0.0, MIN_V) {
            v.linvel.x += MIN_V * v.linvel.x.signum();
        }
    }
}

fn limit_ball_velocity(mut query: Query<(&mut Velocity, &Ball)>) {
    for (mut v, ball) in query.iter_mut() {
        v.linvel = v.linvel.min(ball.max_speed);
    }
}
