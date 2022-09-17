use bevy::prelude::*;

#[derive(Component)]
pub struct Heart {
    pub size_limits: (f32, f32), // min / max
    pub rate: f32,
    pub growing: bool,
}

impl Heart {
    pub fn default() -> Self {
        Heart {
            size_limits: (0.15, 0.5),
            rate: 0.01,
            growing: true,
        }
    }
}

pub fn heart_animation(
    time: Res<Time>,
    mut query: Query<(&mut Heart, &mut Transform)>,
    // keyboard: Res<Input<KeyCode>>,
) {
    for (mut heart, mut transform) in &mut query.iter_mut() {

        /*
        if keyboard.just_pressed(KeyCode::Up) {
                transform.scale *= 1.25;
        } else if keyboard.just_pressed(KeyCode::Down) {
                transform.scale /= 1.25;
        }
        */

        heart.growing = {
            if transform.scale.x > heart.size_limits.1 {
                false
            } else if transform.scale.x < heart.size_limits.0 {
                true
            } else {
                heart.growing
            }
        };

        match heart.growing {
            true => transform.scale += Vec3::splat(heart.rate),
            false => transform.scale -= Vec3::splat(heart.rate),

        };
    };
}