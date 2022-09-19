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