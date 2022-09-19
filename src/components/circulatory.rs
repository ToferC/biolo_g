use bevy::prelude::*;

const HEART_SIZE_LIMITS: (f32, f32) = (0.5, 1.); // Min / Max scale

#[derive(Component)]
pub struct Heart {
    pub rate: f32,
    pub expansion: bool,
}

impl Heart {
    pub fn default() -> Self {
        Heart {
            rate: 1.,
            expansion: true,
        }
    }
}

pub fn heart_animation(
    time: Res<Time>,
    mut query: Query<(&mut Heart, &mut Transform)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    keyboard: Res<Input<KeyCode>>,
) {

    let heart_beat = asset_server.load("sounds/heartbeat.mp3_.ogg");

    for (mut heart, mut transform) in &mut query.iter_mut() {

        
        if keyboard.just_pressed(KeyCode::Up) {
                heart.rate += 0.1;
        } else if keyboard.just_pressed(KeyCode::Down) {
                heart.rate -= 0.1;
        }

        heart.expansion = {
            if transform.scale.x > HEART_SIZE_LIMITS.1 {
                false
            } else if transform.scale.x < HEART_SIZE_LIMITS.0 {
                audio.play(heart_beat.clone());
                true
            } else {
                heart.expansion
            }
        };

        match heart.expansion {
            true => transform.scale += Vec3::splat(heart.rate * time.delta_seconds()),
            false => transform.scale -= Vec3::splat(heart.rate * time.delta_seconds()),

        };
    };
}