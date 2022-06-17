use super::game_entities::*;
use bevy::prelude::*;

const FONT_ASSET: &str = "fonts/Roboto-Regular.ttf";

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_bundle = TextBundle {
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
    };

    commands
        .spawn_bundle(NodeBundle {
            visibility: Visibility { is_visible: false },
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                },
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::FlexEnd,
                margin: Rect {
                    top: Val::Percent(2.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(text_bundle.clone())
                .insert(ScoreText { team: Team::Player });

            parent
                .spawn_bundle(text_bundle.clone())
                .insert(BallLaunchTimerText);

            parent
                .spawn_bundle(text_bundle.clone())
                .insert(ScoreText { team: Team::AI });
        });
}
