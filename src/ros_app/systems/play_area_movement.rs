use bevy::prelude::*;
use glam::{DMat2, DVec2};

use crate::ros_app::{OriginDirection, Panel, PlayArea};
use crate::AppState;

pub fn play_area_movement(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut play_area: ResMut<PlayArea>,
    mut query: Query<(Entity, &Panel, &mut Transform, &mut Sprite)>,
) {
    match app_state.current() {
        AppState::Intro => {} // AppState::InGame => {}
    };
    let mut speed: f64 = 5.;
    let mut direction: f64 = 0.;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.;
    }

    if direction == 0. {
        let closest_cardinal_direction = OriginDirection::closest(play_area.orientation());
        let angle_between = closest_cardinal_direction
            .origin()
            .angle_between(play_area.orientation);
        if angle_between.abs() > 0.01 {
            direction = if angle_between < 0. { 1. } else { -1. };
            speed = 2.5;
        }
    }

    if direction == 0. {
        return;
    }

    // rotate
    let θ: f64 = direction * speed * std::f64::consts::PI / 180.;
    let rotated_orientation: DVec2 = DMat2::from_angle(θ) * play_area.orientation();
    play_area.set_orientation(rotated_orientation);

    for (_panel_entity, panel_object, mut panel_transform, mut panel_sprite) in query.iter_mut() {
        let panel = play_area.panels[panel_object.origin_direction as usize];
        *panel_transform = panel.as_transform();
        *panel_sprite = panel.as_sprite();
    }
}
