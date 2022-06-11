use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct PongInput;
impl Plugin for PongInput {
    fn build(&self, app: &mut App) {
        app.add_startup_system(register_axes).add_system_set(
            SystemSet::new()
                .label(Label::Default)
                .with_system(gather_input)
                .with_system(bevy::input::system::exit_on_esc_system),
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Label {
    Default,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Axis {
    Vertical = 0,
}
impl Default for Axis {
    fn default() -> Self {
        Axis::Vertical
    }
}

#[derive(Default, Component)]
pub struct InputAxes {
    pub val: HashMap<Axis, InputAxis>,
}

#[derive(Default)]
pub struct InputAxis {
    pub val: f32,
    pub axis_id: Axis,
    pub positive_key_codes: HashSet<KeyCode>,
    pub negative_key_codes: HashSet<KeyCode>,
}

impl InputAxis {
    fn set_val(&mut self, v: f32) {
        self.val = v.clamp(-1.0, 1.0);
    }
}

fn register_axes(mut commands: Commands) {
    let vertical_axis = InputAxis {
        axis_id: Axis::Vertical,
        positive_key_codes: HashSet::from([KeyCode::Up, KeyCode::W]),
        negative_key_codes: HashSet::from([KeyCode::Down, KeyCode::S]),
        ..default()
    };

    let axes = InputAxes {
        val: HashMap::from([(vertical_axis.axis_id, vertical_axis)]),
    };

    commands.spawn().insert(axes);
}

fn gather_input(
    keyboard_input: Res<bevy::input::Input<KeyCode>>,
    mut input_axes: Query<&mut InputAxes>,
) {
    for mut axes in input_axes.iter_mut() {
        for axis in axes.val.values_mut() {
            let positive = axis
                .positive_key_codes
                .iter()
                .any(|k| keyboard_input.pressed(*k)) as i32 as f32;
            let negative = axis
                .negative_key_codes
                .iter()
                .any(|k| keyboard_input.pressed(*k)) as i32 as f32;

            axis.set_val(positive - negative);
        }
    }
}
