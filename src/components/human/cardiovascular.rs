use bevy::prelude::*;

use crate::GameElements;

const HEART_SIZE_LIMITS: (f32, f32) = (0.5, 1.); // Min / Max scale

#[derive(Component)]
pub struct Heart {
    pub rate: f32,
    pub expansion: bool,
    pub energy_cost: f32,
    pub efficency: f32,
}

impl Heart {
    pub fn default() -> Self {
        Heart {
            rate: 1.,
            expansion: true,
            energy_cost: 0.01,
            efficency: 1.0,
        }
    }
}

pub fn heart_animation(
    time: Res<Time>,
    mut query: Query<(&mut Heart, &mut Transform)>,
    game_elements: Res<GameElements>,
    audio: Res<Audio>,
    keyboard: Res<Input<KeyCode>>,
) {

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
                audio.play(game_elements.heart_sound.clone());
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