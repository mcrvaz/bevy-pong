use super::game_entities::*;
use bevy::prelude::*;

const FONT_ASSET: &str = "fonts/Roboto-Regular.ttf";

pub fn spawn_score_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(5.0),
                    left: Val::Percent(30.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load(FONT_ASSET),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..default()
        })
        .insert(ScoreText { team: Team::Player });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(5.0),
                    right: Val::Percent(30.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load(FONT_ASSET),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..default()
        })
        .insert(ScoreText { team: Team::AI });
}

pub fn spawn_ball_launch_timer_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Auto,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(5.0),
                    left: Val::Percent(50.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load(FONT_ASSET),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..default()
        })
        .insert(BallLaunchTimerText);
}
