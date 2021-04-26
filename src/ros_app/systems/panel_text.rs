use bevy::prelude::*;

use crate::ros_app::{PanelText, PlayArea};

pub fn panel_text(play_area: Res<PlayArea>, mut query: Query<(&mut Text, &PanelText)>) {
    for (mut text, panel_text) in query.iter_mut() {
        let panel = play_area.panels[panel_text.origin_direction as usize];
        text.sections[2].value = PanelText::format_panel_values(&panel);
    }
}
