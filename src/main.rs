use bevy::{prelude::*};
use bevy::time::FixedTimestep;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use biolo_g::{Game, setup_system, setup_cameras};
use biolo_g::components::circulatory::heart_animation;
use biolo_g::systems::ui::{ui_text_system, text_update_system};

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Biolo_g".to_string(),
            width: 1920.0,
            height: 1280.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_system)
        .add_startup_system(setup_cameras)
        .add_system(heart_animation)
        .add_system(ui_text_system)
        .add_system_set(SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.5))
            .with_system(text_update_system)
        )
        .run();
}
