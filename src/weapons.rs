use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct ShootTimer(pub Timer);