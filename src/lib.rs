use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::circulatory::Heart;
use components::respiratory::Lungs;

#[derive(Default)]
struct PlayerState;

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

const HEART_RADIUS: f32 = 0.5;

const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;

const RESET_FOCUS: [f32; 3] = [
    BOARD_SIZE_I as f32 / 2.0,
    0.0,
    BOARD_SIZE_J as f32 / 2.0 - 0.5,
];

#[derive(Default)]
pub struct Game {
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut windows: ResMut<Windows>,
) {

    // heart
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere::from(shape::Icosphere {
            radius: HEART_RADIUS,
            ..Default::default()
        }))),
        material: materials.add(Color::rgb(1., 0.0, 0.0).into()),
        transform: Transform::from_xyz(-2.0, 1.0, 0.0),
        ..Default::default()
    })
    .insert(Heart::default());

    // lungs
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Torus::from(shape::Torus {
            radius: 0.75,
            ..Default::default()
        }))),
        material: materials.add(Color::rgb(0., 0.25, 0.95).into()),
        transform: Transform::from_xyz(-2., 0., 0.),
        ..Default::default()
    })
    .insert(Lungs::default());

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight { 
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });


    // capture window size
    /*
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // position window
    window.set_position(IVec2::new(2780, 4900));

    // add Winsize Resource
    let win_size = WinSize { w: win_w, h: win_h};
    commands.insert_resource(win_size);
    */

 }

 pub fn setup_cameras(mut commands: Commands) {
    
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
 }
