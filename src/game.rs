use super::{game_entities::*, game_scene_setup::*, input, utils};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PongGame;
impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_event::<GoalEvent>()
            .add_startup_system(setup_camera)
            .add_startup_system(setup_physics)
            .add_startup_system(spawn_ball)
            .add_startup_system(spawn_paddles)
            .add_startup_system(spawn_bounds)
            .add_startup_system(spawn_score)
            .add_startup_system(start_ball_movement)
            .add_system_set(
                SystemSet::new()
                    .label(Label::CollisionCheck)
                    .after(input::Label::Default)
                    .with_system(evaluate_ball_collision),
            )
            .add_system_set(
                SystemSet::new()
                    .label(Label::Default)
                    .after(Label::CollisionCheck)
                    .with_system(score)
                    .with_system(reset_ball)
                    .with_system(paddle_movement),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    CollisionCheck,
    Default,
}

fn start_ball_movement(mut query: Query<(&Ball, &mut Velocity, &mut Transform)>) {
    for (ball, mut velocity, mut transform) in query.iter_mut() {
        set_initial_ball_position(&mut transform);
        set_initial_ball_speed(ball, &mut velocity);
    }
}

fn paddle_movement(
    axis_inputs: Query<&input::InputAxes>,
    mut query: Query<(&mut Velocity, &Paddle), With<PlayerPaddle>>,
) {
    let input = axis_inputs.single();
    let vertical_input = input.val.get(&input::Axis::Vertical).unwrap();
    for (mut rb, paddle) in query.iter_mut() {
        rb.linvel = Vec2::new(0.0, vertical_input.val * paddle.speed);
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
        let team = &ev.team;
        let team_score = match_score.score.get_mut(team).unwrap();
        *team_score += 1;
    }
}

fn reset_ball(
    mut ev_goal: EventReader<GoalEvent>,
    mut ball_query: Query<(Entity, &Ball, &mut Velocity, &mut Transform)>,
) {
    for ev in ev_goal.iter() {
        let (_, ball, mut velocity, mut transform) = ball_query
            .iter_mut()
            .find(|x| x.0.id() == ev.ball_id)
            .unwrap();
        set_initial_ball_position(&mut transform);
        set_initial_ball_speed(ball, &mut velocity);
    }
}

fn set_initial_ball_position(mut transform: &mut Transform) {
    transform.translation = Vec3::ZERO;
}

fn set_initial_ball_speed(ball: &Ball, mut velocity: &mut Velocity) {
    velocity.linvel = utils::random_horizontal() * ball.initial_speed;
}
