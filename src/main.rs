use bevy::{prelude::*};
use biolo_g::{Game, setup_system, setup_cameras};
use biolo_g::components::heart::heart_animation;

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
        .add_startup_system(setup_system)
        .add_startup_system(setup_cameras)
        .add_system(heart_animation)
        .run();
}
