use bevy::{prelude::*};
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

fn main() {
    App::new()
        .insert_resource(ShootTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins((DefaultPlugins, Wireframe2dPlugin::default()))
        .add_systems(Startup, (spawn_text, spawn_player))
        .add_systems(Update, toggle_wireframe)
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(
            // The `RunFixedMainLoop` schedule allows us to schedule systems to run before and after the fixed timestep loop.
            RunFixedMainLoop,
            (
                // The physics simulation needs to know the player's input, so we run this before the fixed timestep loop.
                // Note that if we ran it in `Update`, it would be too late, as the physics simulation would already have been advanced.
                // If we ran this in `FixedUpdate`, it would sometimes not register player input, as that schedule may run zero times per frame.
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                handle_rotation.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                // The player's visual representation needs to be updated after the physics simulation has been advanced.
                // This could be run in `Update`, but if we run it here instead, the systems in `Update`
                // will be working with the `Transform` that will actually be shown on screen.
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        )
        .run();
}

/// Since Bevy's default 2D camera setup is scaled such that
/// one unit is one pixel, you can think of this as
/// "How many pixels per second should the player move?"
const SHIP_SPEED: f32 = 500.;
/// Radians per Second
const ROTATION_SPEED: f32 = 3.;

#[derive(Component)]
enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Tracks the ships rotation in the physics simulation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PhysicalRotation(f32);

/// The value [`PhysicalRotation`] had in the last fixed timestep.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PreviousPhysicalRotation(f32);

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct AccumulatedInput(Vec2);

/// A vector representing the player's velocity in the physics simulation.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct Velocity(Vec3);

/// The actual position of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual representation.
///
/// If you want to make sure that this component is always initialized
/// with the same value as the `Transform`'s translation, you can
/// use a [component lifecycle hook](https://docs.rs/bevy/0.14.0/bevy/ecs/component/struct.ComponentHooks.html)
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PhysicalTranslation(Vec3);

/// The value [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

#[derive(Resource)]
struct ShootTimer(Timer);

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Collider)]
struct Ship;

#[derive(Component)]
struct Name(String);

/// Spawn a bit of UI text to explain how to move the player.
fn spawn_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Press space to toggle wireframes"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
        .with_child((
            Text::new("Move the player with WASD"),
            TextFont {
                font_size: 25.0,
                ..default()
            },
        ));
}

/// Spawn the player sprite and a 2D camera.
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let ship = meshes.add(Triangle2d::new(Vec2::Y * 50., Vec2::new(-50., -50.), Vec2::new(50., -50.)));
    let ship_color = Color::srgb(0.0, 0.0, 1.0);
    
    commands.spawn(Camera2d);
    commands.spawn((
        Name("Player".to_string()),
        // Sprite::from_image(asset_server.load("branding/icon.png")),
        Mesh2d(ship),
        MeshMaterial2d(materials.add(ship_color)),
        Transform::from_scale(Vec3::splat(0.3)),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
        PhysicalRotation::default(),
        PreviousPhysicalRotation::default(),
        Ship,
        Collider,
    ));
}

fn toggle_wireframe(mut wireframe_config: ResMut<Wireframe2dConfig>, keyboard: Res<ButtonInput<KeyCode>>, ) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&mut AccumulatedInput, &mut Velocity, &PhysicalRotation)>
) {
    for (mut input, mut velocity, rotation) in query.iter_mut() {
        // Handle forward/backward movement with W/S
        if keyboard_input.pressed(KeyCode::KeyW) {
            // Move forward in the direction the ship is facing
            let forward = Vec2::new(rotation.0.sin(), rotation.0.cos());
            input.0 += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            // Move backward in the direction the ship is facing
            let backward = Vec2::new(-rotation.0.sin(), -rotation.0.cos());
            input.0 += backward;
        }
        
        // A/D will be handled in a separate rotation system
        
        // Normalize and scale the velocity
        velocity.0 = input.extend(0.0).normalize_or_zero() * SHIP_SPEED;
    }
}

fn handle_rotation(
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

/// Advance the physics simulation by one fixed timestep. This may run zero or multiple times per frame.
///
/// Note that since this runs in `FixedUpdate`, `Res<Time>` would be `Res<Time<Fixed>>` automatically.
/// We are being explicit here for clarity.
fn advance_physics(fixed_time: Res<Time<Fixed>>, mut query: Query<(&mut PhysicalTranslation, &mut PreviousPhysicalTranslation, &mut AccumulatedInput, &Velocity,)>) {
    for (mut current_physical_translation, mut previous_physical_translation, mut input, velocity) in query.iter_mut()
    {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();

        // Reset the input accumulator, as we are currently consuming all input that happened since the last fixed timestep.
        input.0 = Vec2::ZERO;
    }
}

fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
        &PhysicalRotation,
        &PreviousPhysicalRotation,
    )>,
) {
    for (mut transform, current_translation, previous_translation, current_rotation, previous_rotation) in
        query.iter_mut()
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