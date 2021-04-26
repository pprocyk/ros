#![allow(mixed_script_confusables)]

use bevy::prelude::*;

pub mod ros_app;
use ros_app::{systems, PlayArea};

pub const APP_TITLE: &str = "ros";
pub const APP_SCREEN_WIDTH: f32 = 1280_f32;
pub const APP_SCREEN_HEIGHT: f32 = 720_f32;

// #[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
// pub enum AppSystem {
//     World,
// }

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Intro,
    // InGame,
}

fn main() {
    let mut app = App::build();

    app.add_state(AppState::Intro);

    app.insert_resource(WindowDescriptor {
        title: APP_TITLE.to_string(),
        width: APP_SCREEN_WIDTH,
        height: APP_SCREEN_HEIGHT,
        ..Default::default()
    });

    app.insert_resource(ClearColor(Color::BLACK));

    // app
    //     .insert_resource(bevy::ecs::schedule::ReportExecutionOrderAmbiguities)
    //     .add_plugin(bevy_diagnostic::LogDiagnosticsPlugin::default())
    //     .add_plugin(bevy_diagnostic::FrameTimeDiagnosticsPlugin::default());

    app //.add_plugins(DefaultPlugins);
        .add_plugin(bevy_log::LogPlugin::default())
        .add_plugin(bevy_core::CorePlugin::default())
        .add_plugin(bevy_transform::TransformPlugin::default())
        .add_plugin(bevy_diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy_input::InputPlugin::default())
        .add_plugin(bevy_window::WindowPlugin::default())
        .add_plugin(bevy_asset::AssetPlugin::default())
        .add_plugin(bevy_scene::ScenePlugin::default())
        .add_plugin(bevy_render::RenderPlugin::default())
        .add_plugin(bevy_sprite::SpritePlugin::default())
        .add_plugin(bevy_pbr::PbrPlugin::default())
        .add_plugin(bevy_ui::UiPlugin::default())
        .add_plugin(bevy_text::TextPlugin::default())
        .add_plugin(bevy_audio::AudioPlugin::default())
        .add_plugin(bevy_gilrs::GilrsPlugin::default())
        .add_plugin(bevy_gltf::GltfPlugin::default())
        .add_plugin(bevy_winit::WinitPlugin::default())
        .add_plugin(bevy_wgpu::WgpuPlugin::default());

    app.add_startup_system(camera_setup.system());
    app.insert_resource(PlayArea::new(
        APP_SCREEN_WIDTH.into(),
        APP_SCREEN_HEIGHT.into(),
    ))
    .add_startup_system(systems::panel_setup.system())
    .add_system(systems::play_area_movement.system())
    .add_system(systems::orientation_text.system())
    .add_system(systems::panel_text.system());

    app.run();
}

fn camera_setup(mut commands: Commands) {
    // ui camera?
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
