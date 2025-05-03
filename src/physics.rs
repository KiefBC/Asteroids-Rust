use bevy::prelude::*;
use crate::player::ROTATION_SPEED;

pub const SHIP_SPEED: f32 = 500.;

/// Tracks the ships rotation in the physics simulation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalRotation(pub f32);

/// The value [`PhysicalRotation`] had in the last fixed timestep.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalRotation(pub f32);

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(pub Vec2);

/// A vector representing the player's velocity in the physics simulation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

/// The actual position of the player in the physics simulation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec3);

/// The value [`PhysicalTranslation`] had in the last fixed timestep.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec3);

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&mut AccumulatedInput, &mut Velocity, &PhysicalRotation)>
) {
    for (mut input, mut velocity, rotation) in query.iter_mut() {
        // Transform the ship's local up vector (0,1) by the current rotation,
        // so forward is always the nose direction.
        let forward = Vec2::new(-rotation.0.sin(), rotation.0.cos());
        // Handle forward/backward movement with W/S
        if keyboard_input.pressed(KeyCode::KeyW) {
            // Move forward in the direction the ship is facing
            // For Z-axis rotation, forward is (cos(θ), sin(θ))
            input.0 += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            // Move backward in the direction the ship is facing
            input.0 -= forward;
        }

        // Normalize and scale the velocity
        velocity.0 = input.extend(0.0).normalize_or_zero() * SHIP_SPEED;
    }
}

pub fn handle_rotation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut PhysicalRotation, &mut PreviousPhysicalRotation)>
) {
    for (mut rotation, mut prev_rotation) in query.iter_mut() {
        prev_rotation.0 = rotation.0;

        if keyboard_input.pressed(KeyCode::KeyA) {
            // Rotate counterclockwise
            rotation.0 += ROTATION_SPEED * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            // Rotate clockwise
            rotation.0 -= ROTATION_SPEED * time.delta_secs();
        }
    }
}

pub fn advance_physics(
    fixed_time: Res<Time<Fixed>>, 
    mut query: Query<(&mut PhysicalTranslation, &mut PreviousPhysicalTranslation, &mut AccumulatedInput, &Velocity)>
) {
    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input, velocity
    ) in query.iter_mut()
    {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();

        // Reset the input accumulator, as we are currently consuming all input that happened since the last fixed timestep.
        input.0 = Vec2::ZERO;
    }
}

pub fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &PhysicalTranslation, &PreviousPhysicalTranslation, &PhysicalRotation, &PreviousPhysicalRotation)>
) {
    for (
        mut transform,
        current_translation,
        previous_translation,
        current_rotation,
        previous_rotation
    ) in query.iter_mut()
    {
        let alpha = fixed_time.overstep_fraction();

        // Interpolate position
        let previous_pos = previous_translation.0;
        let current_pos = current_translation.0;
        let rendered_translation = previous_pos.lerp(current_pos, alpha);

        // Interpolate rotation
        let previous_rot = previous_rotation.0;
        let current_rot = current_rotation.0;
        let rendered_rotation = previous_rot + alpha * (current_rot - previous_rot);

        // Apply to transform
        transform.translation = rendered_translation;
        transform.rotation = Quat::from_rotation_z(rendered_rotation);
    }
}