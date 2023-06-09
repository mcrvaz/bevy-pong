use super::{
    game_entities::*, game_setup_systems::*, game_systems::*, game_ui_setup_systems::*,
    game_ui_systems::*, input,
};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Setup,
    CollisionCheck,
    BallLaunch,
    Default,
    UI,
}

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
                    .with_system(spawn_hud),
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
                    .with_system(prevent_stuck_ball.run_if(was_ball_launched))
                    .with_system(score)
                    .with_system(reset_ball)
                    .with_system(paddle_movement)
                    .with_system(enemy_paddle_movement)
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
