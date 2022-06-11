use super::{game_entities::*, game_scene_setup::*, input, utils};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PongGame;
impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(setup_camera)
            .add_startup_system(setup_physics)
            .add_startup_system(spawn_ball)
            .add_startup_system(spawn_paddles)
            .add_startup_system(spawn_bounds)
            .add_startup_system(spawn_score)
            .add_event::<GoalEvent>()
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
                    // .with_system(ball_movement)
                    .with_system(score)
                    .with_system(reset_ball)
                    .with_system(paddle_movement),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Default,
    CollisionCheck,
}

fn paddle_movement(
    axis_inputs: Query<&input::InputAxes>,
    mut query: Query<(&mut Velocity, &Speed), With<PlayerPaddle>>,
) {
    let input = axis_inputs.single();
    let vertical_input = input.val.get(&input::Axis::Vertical).unwrap();
    for (mut rb, speed) in query.iter_mut() {
        rb.linvel = Vec2::new(0.0, vertical_input.val * speed.current);
    }
}

// fn ball_movement(mut query: Query<(&mut Velocity, &MovementDirection, &Speed), With<Ball>>) {
//     for (mut rb, mov_dir, speed) in query.iter_mut() {
//         rb.linvel = (mov_dir.0 * speed.current).truncate();
//     }
// }

// fn start_ball_movement(mut query: Query<(&mut Velocity, &MovementDirection, &Speed), With<Ball>>) {
//     for (mut rb, mov_dir, speed) in query.iter_mut() {
//         println!("{}", "alo");
//         rb.linvel = Vec2::new(1.0 * 500.0, 0.0);
//     }
// }

fn evaluate_ball_collision(
    mut ball_query: Query<(Entity, &mut MovementDirection), With<Ball>>,
    rapier_context: Res<RapierContext>,
) {
    for (ball_entity, mut ball_mov_dir) in ball_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(ball_entity) {
            for manifold in contact_pair.manifolds() {
                // ball_mov_dir.0 = manifold.normal().extend(0.0);
            }
        }
    }
}

fn handle_ball_collision(
    collision_vectors: &[Vec3],
    ball_mov_dir: &mut MovementDirection,
    ball_speed: &mut Speed,
    ball: &Ball,
) {
    let result_dir = utils::avg(collision_vectors);
    // ball_mov_dir.0 = result_dir.normalize_or_zero();
    ball_speed.current *= ball.speed_multiplier;
}

fn handle_ball_goal_collision(mut ev_goal: EventWriter<GoalEvent>, goal: &Goal, ball_id: u32) {
    ev_goal.send(GoalEvent {
        team: goal.team.opposite(),
        ball_id: ball_id,
    });
}

fn score(mut ev_goal: EventReader<GoalEvent>, mut query: Query<&mut MatchScore>) {
    let mut match_score = query.single_mut();
    for ev in ev_goal.iter() {
        let team = &ev.team;
        let team_score = match_score.score.get_mut(team).unwrap();
        *team_score += 1;
    }
}

fn reset_ball(
    mut ev_goal: EventReader<GoalEvent>,
    mut ball_query: Query<(Entity, &mut Transform, &mut MovementDirection, &mut Speed), With<Ball>>,
) {
    for ev in ev_goal.iter() {
        let (_, mut transform, mut mov_dir, mut speed) = ball_query
            .iter_mut()
            .find(|x| x.0.id() == ev.ball_id)
            .unwrap();
        transform.translation = Vec3::ZERO;
        mov_dir.set_random_horizontal();
        speed.reset();
    }
}
