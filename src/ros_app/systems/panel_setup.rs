use bevy::prelude::*;

use crate::ros_app::{OrientationText, OriginDirection, Panel, PanelText, PlayArea, PlayAreaPanel};

pub fn panel_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    play_area: Res<PlayArea>,
) {
    let font_fira_sans_bold: &str = "fonts/fira/FiraSans-Bold.ttf";
    let font_fira_mono_medium: &str = "fonts/fira/FiraMono-Medium.ttf";

    let panel_colors: [Color; 4] = [
        Color::MIDNIGHT_BLUE,
        Color::SILVER,
        Color::GOLD,
        Color::GRAY,
    ];

    let panels: &[PlayAreaPanel; 4] = play_area.panels();

    // panel 0
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(panel_colors[0].into()),
            transform: panels[0].as_transform(),
            sprite: panels[0].as_sprite(),
            ..Default::default()
        })
        .insert(Panel {
            origin_direction: OriginDirection::Up,
        });
    // panel 1
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(panel_colors[1].into()),
            transform: panels[1].as_transform(),
            sprite: panels[1].as_sprite(),
            ..Default::default()
        })
        .insert(Panel {
            origin_direction: OriginDirection::Right,
        });
    // panel 2
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(panel_colors[2].into()),
            transform: panels[2].as_transform(),
            sprite: panels[2].as_sprite(),
            ..Default::default()
        })
        .insert(Panel {
            origin_direction: OriginDirection::Down,
        });
    // panel 3
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(panel_colors[3].into()),
            transform: panels[3].as_transform(),
            sprite: panels[3].as_sprite(),
            ..Default::default()
        })
        .insert(Panel {
            origin_direction: OriginDirection::Left,
        });

    for (panel_num, panel) in panels.iter().enumerate() {
        commands
            .spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Panel ".to_string(),
                            style: TextStyle {
                                font: asset_server.load(font_fira_sans_bold),
                                font_size: 40.0,
                                color: Color::rgb(0.5, 0.5, 1.0),
                            },
                        },
                        TextSection {
                            value: format!("{} ", panel_num),
                            style: TextStyle {
                                font: asset_server.load(font_fira_mono_medium),
                                font_size: 40.0,
                                color: panel_colors[panel_num],
                            },
                        },
                        TextSection {
                            value: PanelText::format_panel_values(panel),
                            style: TextStyle {
                                font: asset_server.load(font_fira_mono_medium),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(40_f32 * (panel_num as f32 + 1_f32) + 5_f32),
                        left: Val::Px(5_f32),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PanelText {
                origin_direction: panel.origin_direction,
            });
    }

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Orientation: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load(font_fira_sans_bold),
                            font_size: 40.0,
                            color: Color::rgb(0.5, 0.5, 1.0),
                        },
                    },
                    TextSection {
                        value: OrientationText::format_orientation(play_area.orientation()),
                        style: TextStyle {
                            font: asset_server.load(font_fira_mono_medium),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 0.5, 0.5),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5_f32),
                    left: Val::Px(5_f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(OrientationText);
}
