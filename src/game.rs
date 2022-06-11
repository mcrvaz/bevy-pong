use super::{game_entities::*, game_scene_setup::*, input, utils};
use bevy::{prelude::*, sprite::collide_aabb};

pub struct PongGame;
impl Plugin for PongGame {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(setup_camera)
            .add_startup_system(spawn_ball)
            .add_startup_system(spawn_paddles)
            .add_startup_system(spawn_bounds)
            .add_startup_system(spawn_score)
            .add_event::<GoalEvent>()
            .add_system_set(
                SystemSet::new()
                    .label(Label::Default)
                    .after(input::Label::Default)
                    .with_system(paddle_movement)
                    .with_system(ball_collision)
                    .with_system(ball_movement)
                    .with_system(score)
                    .with_system(reset_ball),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Default,
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
        transform.translation.y += vertical_input.val * speed.current * time.delta_seconds();
        mov_dir.0 = (transform.translation - previous_pos).normalize_or_zero();
    }
}

fn ball_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MovementDirection, &Speed), With<Ball>>,
) {
    for (mut transform, move_dir, speed) in query.iter_mut() {
        transform.translation += move_dir.0 * speed.current * time.delta_seconds();
    }
}

fn ball_collision(
    ev_goal: EventWriter<GoalEvent>,
    mut ball_query: Query<(
        Entity,
        &Sprite,
        &Transform,
        &Ball,
        &mut MovementDirection,
        &mut Speed,
    )>,
    colliders_query: Query<
        (
            &Sprite,
            &Transform,
            Option<&MovementDirection>,
            Option<&Goal>,
        ),
        Without<Ball>,
    >,
) {
    for (entity, ball_spr, ball_transform, ball, mut ball_mov_dir, mut ball_speed) in
        ball_query.iter_mut()
    {
        for (coll_spr, coll_transform, coll_mov_dir, goal) in colliders_query.iter() {
            let collision = collide_aabb::collide(
                ball_transform.translation,
                ball_spr.custom_size.unwrap(),
                coll_transform.translation,
                coll_spr.custom_size.unwrap(),
            );
            if let Some(c) = collision {
                if let Some(g) = goal {
                    handle_ball_goal_collision(ev_goal, g, entity.id());
                    return;
                }

                let collision_dir = utils::v2_to_v3(utils::collision_to_direction(c) * 2.0);
                let vectors = if let Some(c_mov_dir) = coll_mov_dir {
                    vec![collision_dir, ball_mov_dir.0, c_mov_dir.0]
                } else {
                    vec![collision_dir, ball_mov_dir.0]
                };
                handle_ball_collision(&vectors, &mut ball_mov_dir, &mut ball_speed, ball);
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
    ball_mov_dir.0 = result_dir.normalize_or_zero();
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
