use bevy::prelude::*;

use crate::ros_app::{OrientationText, PlayArea};

pub fn orientation_text(
    play_area: Res<PlayArea>,
    mut query: Query<&mut Text, With<OrientationText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        text.sections[1].value = OrientationText::format_orientation(play_area.orientation());
    }
}
