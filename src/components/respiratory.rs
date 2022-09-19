use bevy::prelude::*;

const LUNGS_SIZE: (f32, f32) = (0.5, 1.);

#[derive(Component)]
pub struct Lungs {
    pub expansion: bool,
    pub rate: f32, 
}

impl Lungs {
    pub fn default() -> Self {
        Lungs {
            expansion: true,
            rate: 0.75,
        }
    }
}

pub fn breathe_system(
    time: Res<Time>,
    mut query: Query<(&mut Lungs, &mut Transform)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    keyboard: Res<Input<KeyCode>>,
) {
    let breathing = asset_server.load("sounds/breathe.ogg");

    for (mut lungs, mut transform) in &mut query.iter_mut() {
        
        if keyboard.just_pressed(KeyCode::Right) {
            lungs.rate += 0.1;
        } else if keyboard.just_pressed(KeyCode::Left) {
            lungs.rate -= 0.1;
        }
        
        lungs.expansion = match transform.scale.x {
            x if x > LUNGS_SIZE.1 => false,
            x if x < LUNGS_SIZE.0 => {
                audio.play(breathing.clone());
                true
            },
            _ => lungs.expansion
        };

        match lungs.expansion {
            true => transform.scale += Vec3::splat(lungs.rate * time.delta_seconds()),
            false => transform.scale -= Vec3::splat(lungs.rate * time.delta_seconds()),
        }
    }
}