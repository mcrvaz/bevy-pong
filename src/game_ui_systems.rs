use super::game_entities::*;
use bevy::prelude::*;

pub fn update_ball_launch_timer(
    timer: Res<BallLaunchDelay>,
    mut query: Query<(&mut Style, &mut Text), With<BallLaunchTimerText>>,
) {
    for (mut style, mut text) in query.iter_mut() {
        if timer.0.finished() {
            style.display = Display::None;
        } else {
            style.display = Display::Flex;
            let remaining = timer.0.duration().as_secs_f32() - timer.0.elapsed_secs();
            text.sections[0].value = format!("{:0.1}", remaining);
        }
    }
}

pub fn initial_score(
    score_query: Query<&MatchScore>,
    mut text_query: Query<(&ScoreText, &mut Text)>,
) {
    update_score(&score_query, &mut text_query);
}

pub fn update_score_runtime(
    mut ev_goal: EventReader<GoalEvent>,
    score_query: Query<&MatchScore>,
    mut text_query: Query<(&ScoreText, &mut Text)>,
) {
    for _ in ev_goal.iter() {
        update_score(&score_query, &mut text_query);
    }
}

fn update_score(score_query: &Query<&MatchScore>, text_query: &mut Query<(&ScoreText, &mut Text)>) {
    let match_score = score_query.single();
    for (score_text, mut text) in text_query.iter_mut() {
        let team_score = match_score.score.get(&score_text.team).unwrap();
        text.sections[0].value = team_score.to_string();
    }
}
