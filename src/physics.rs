use bevy::prelude::*;

/// Ship thrust force applied when accelerating.
/// This is the force applied to the ship when thrust input is detected.
pub const THRUST_FORCE: f32 = 800.0;

/// Maximum ship velocity in pixels per second.
/// This caps the ship's maximum speed to prevent runaway acceleration.
pub const MAX_VELOCITY: f32 = 600.0;

/// Linear dampening factor for space-like physics.
/// This simulates drag/friction in space. Values closer to 1.0 mean less dampening.
/// 0.98 means the ship retains 98% of its velocity each second, losing 2% to dampening.
pub const LINEAR_DAMPENING: f32 = 0.98;

/// Screen boundaries for wrapping
pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const WRAP_MARGIN: f32 = 50.0; // How far off-screen before wrapping

/// Ship rotation speed in radians per second.
/// Controls how quickly the ship can turn left or right.
pub const ROTATION_SPEED: f32 = 4.5;

/// Represents the ship's current rotation angle in the physics simulation.
/// Stored in radians, where 0 points upward and rotation increases clockwise.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalRotation(pub f32);

/// Stores the previous frame's rotation value for interpolation.
/// Used to smoothly render rotation between physics updates.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalRotation(pub f32);

/// Represents cardinal movement directions for input handling.
/// Used to translate keyboard input into directional movement.
#[derive(Debug, Component, Clone, Copy, PartialEq)]
pub enum MoveDirection {
    /// Upward movement (W key)
    Up,
    /// Downward movement (S key)
    Down,
    /// Leftward rotation (A key)
    Left,
    /// Rightward rotation (D key)
    Right,
}

/// Defines the interface for accumulating and managing input over time.
/// Allows for consistent input handling across different input types.
pub trait InputAccumulator {
    /// Adds the given input vector to the accumulated value
    fn accumulate(&mut self, input: Vec2);
    /// Clears all accumulated input
    fn reset(&mut self);
    /// Returns the current accumulated input vector
    fn get(&self) -> Vec2;
}

/// Implements input accumulation for ship movement.
/// Collects and stores directional input between physics updates.
#[derive(Component, Default, Debug, Clone)]
pub struct MovementInputAccumulator {
    /// The accumulated movement vector
    pub value: Vec2,
}

impl InputAccumulator for MovementInputAccumulator {
    /// Adds the given input vector to the accumulated movement
    fn accumulate(&mut self, input: Vec2) {
        self.value += input;
    }
    
    /// Resets accumulated movement to zero
    fn reset(&mut self) {
        self.value = Vec2::ZERO;
    }
    
    /// Returns the current accumulated movement vector
    fn get(&self) -> Vec2 {
        self.value
    }
}

/// Represents the ship's current velocity in the physics simulation.
/// Stored as a 3D vector where z is typically zero for 2D movement.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

/// Represents the ship's current position in the physics simulation.
/// This may differ from the rendered position due to interpolation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec3);

/// Stores the previous frame's position for interpolation.
/// Used to smoothly render movement between physics updates.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec3);

/// Bundle of physics components for the ship.
/// Provides a convenient way to add all required physics components at once.
#[derive(Bundle, Default)]
pub struct ShipPhysicsBundle {
    pub physical_translation: PhysicalTranslation,
    pub previous_physical_translation: PreviousPhysicalTranslation,
    pub physical_rotation: PhysicalRotation,
    pub previous_physical_rotation: PreviousPhysicalRotation,
    pub velocity: Velocity,
    pub movement_input_accumulator: MovementInputAccumulator,
}

/// Processes keyboard input and updates movement accumulators.
/// 
/// This system:
/// 1. Detects which direction keys are pressed
/// 2. Converts key presses to movement vectors
/// 3. Accumulates movement input for later processing
/// 
/// Note: Left/Right inputs are handled separately in the rotation system.
pub fn gather_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementInputAccumulator>,
) {
    let directions = get_pressed_directions(&keyboard_input);

    for mut input_accumulator in query.iter_mut() {
        let mut input = Vec2::ZERO;

        for dir in directions.iter() {
            match dir {
                MoveDirection::Up => input.y += 1.0,
                MoveDirection::Down => input.y -= 1.0,
                MoveDirection::Left => { /* handled in handle_rotation */ }
                MoveDirection::Right => { /* handled in handle_rotation */ }
            }
        }
        
        input_accumulator.accumulate(input);
    }
}

/// Converts keyboard input into movement directions.
/// 
/// Maps WASD keys to their corresponding cardinal directions:
/// - W → Up
/// - S → Down
/// - A → Left
/// - D → Right
/// 
/// Returns a vector of all currently pressed directions.
fn get_pressed_directions(keyboard_input: &ButtonInput<KeyCode>) -> Vec<MoveDirection> {
    let mut directions = Vec::new();

    if keyboard_input.pressed(KeyCode::KeyW) {
        directions.push(MoveDirection::Up);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        directions.push(MoveDirection::Down);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        directions.push(MoveDirection::Left);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        directions.push(MoveDirection::Right);
    }

    directions
}

/// Applies thrust force based on input with space-like physics.
/// 
/// This system:
/// 1. Gets the accumulated input vector
/// 2. Calculates thrust direction based on ship's rotation
/// 3. Applies thrust force to current velocity (acceleration)
/// 4. Clamps velocity to maximum speed
/// 5. Applies linear dampening for realistic space movement
pub fn apply_movement(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&MovementInputAccumulator, &PhysicalRotation, &mut Velocity)>,
) {
    let dt = fixed_time.delta_seconds();
    
    for (input_accumulator, rotation, mut velocity) in query.iter_mut() {
        let input = input_accumulator.get();
        
        // Calculate thrust direction based on ship's forward vector
        let forward = Vec2::new(-rotation.0.sin(), rotation.0.cos());
        
        // Apply thrust force when input is detected
        if input.length() > 0.0 {
            // Thrust is applied in the ship's forward direction
            let thrust = forward * input.y * THRUST_FORCE * dt;
            velocity.0 += thrust.extend(0.0);
        }
        
        // Apply linear dampening to simulate space drag
        let dampening_factor = LINEAR_DAMPENING.powf(dt);
        velocity.0 *= dampening_factor;
        
        // Clamp velocity to maximum speed
        let current_speed = velocity.0.length();
        if current_speed > MAX_VELOCITY {
            velocity.0 = velocity.0.normalize() * MAX_VELOCITY;
        }
    }
}

/// Processes rotation input and updates the ship's orientation.
/// 
/// This system:
/// 1. Detects left/right key presses
/// 2. Updates the ship's rotation based on input and elapsed time
/// 3. Stores the previous rotation for interpolation
/// 4. Rotation is immediate and not affected by dampening
pub fn apply_rotation_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut PhysicalRotation, &mut PreviousPhysicalRotation)>,
) {
    for (mut rotation, mut prev_rotation) in query.iter_mut() {
        // Store current rotation for interpolation
        prev_rotation.0 = rotation.0;

        // Apply rotation based on input - immediate response, no dampening
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation.0 += ROTATION_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation.0 -= ROTATION_SPEED * time.delta_seconds();
        }
    }
}

/// Updates physics state at a fixed timestep.
/// 
/// This system:
/// 1. Stores the current position for interpolation
/// 2. Updates position based on velocity and elapsed time
/// 3. Resets input accumulation for the next frame
/// 
/// Running at a fixed timestep ensures consistent physics regardless of framerate.
pub fn update_physics_state(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut MovementInputAccumulator,
        &Velocity,
    )>,
) {
    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input_accumulator,
        velocity,
    ) in query.iter_mut()
    {
        // Store current position for interpolation
        previous_physical_translation.0 = current_physical_translation.0;
        
        // Update position based on velocity
        current_physical_translation.0 += velocity.0 * fixed_time.delta_seconds();

        // Reset input for next frame
        input_accumulator.reset();
    }
}

/// Smoothly interpolates between physics states for rendering.
/// 
/// This system:
/// 1. Calculates interpolation factor based on time between fixed updates
/// 2. Linearly interpolates position between previous and current states
/// 3. Linearly interpolates rotation between previous and current states
/// 4. Updates the transform component used for rendering
/// 
/// This creates smooth visual movement even when physics updates at a fixed rate.
pub fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
        &PhysicalRotation,
        &PreviousPhysicalRotation,
    )>,
) {
    for (
        mut transform,
        current_translation,
        previous_translation,
        current_rotation,
        previous_rotation,
    ) in query.iter_mut()
    {
        // Calculate interpolation factor (0.0 to 1.0)
        let alpha = fixed_time.overstep_fraction();

        // Interpolate position
        let previous_pos = previous_translation.0;
        let current_pos = current_translation.0;
        let rendered_translation = previous_pos.lerp(current_pos, alpha);

        // Interpolate rotation
        let previous_rot = previous_rotation.0;
        let current_rot = current_rotation.0;
        let rendered_rotation = previous_rot + alpha * (current_rot - previous_rot);

        // Apply interpolated values to the rendered transform
        transform.translation = rendered_translation;
        transform.rotation = Quat::from_rotation_z(rendered_rotation);
    }
}

/// Wraps the ship's position when it goes off-screen.
/// 
/// This system:
/// 1. Checks if the ship has moved beyond screen boundaries
/// 2. Wraps the ship to the opposite side of the screen
/// 3. Maintains velocity and rotation during wrapping
pub fn wrap_screen_position(
    mut query: Query<(&mut PhysicalTranslation, &mut PreviousPhysicalTranslation)>,
) {
    for (mut translation, mut prev_translation) in query.iter_mut() {
        let half_width = SCREEN_WIDTH / 2.0;
        let half_height = SCREEN_HEIGHT / 2.0;
        
        // Wrap horizontally
        if translation.0.x > half_width + WRAP_MARGIN {
            translation.0.x = -half_width - WRAP_MARGIN;
            prev_translation.0.x = translation.0.x;
        } else if translation.0.x < -half_width - WRAP_MARGIN {
            translation.0.x = half_width + WRAP_MARGIN;
            prev_translation.0.x = translation.0.x;
        }
        
        // Wrap vertically
        if translation.0.y > half_height + WRAP_MARGIN {
            translation.0.y = -half_height - WRAP_MARGIN;
            prev_translation.0.y = translation.0.y;
        } else if translation.0.y < -half_height - WRAP_MARGIN {
            translation.0.y = half_height + WRAP_MARGIN;
            prev_translation.0.y = translation.0.y;
        }
    }
}

/// Resets the ship to its initial state when the R key is pressed.
/// 
/// This system:
/// 1. Detects when the R key is pressed
/// 2. Resets the ship's position to the origin (0,0,0)
/// 3. Resets the ship's velocity to zero
/// 4. Resets the ship's rotation to the default orientation
pub fn reset_ship_position(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PhysicalTranslation, &mut Velocity, &mut PhysicalRotation)>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for (mut translation, mut velocity, mut rotation) in query.iter_mut() {
            // Reset position to origin
            translation.0 = Vec3::ZERO;
            // Reset velocity to zero
            velocity.0 = Vec3::ZERO;
            // Reset rotation to default (upward facing)
            rotation.0 = 0.0;
        }
    }
}