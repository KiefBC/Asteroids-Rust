use asteroids_rust::particles::{Particle, engine_particle_system};
use asteroids_rust::physics::{
    MAX_VELOCITY, MovementInputAccumulator, PhysicalRotation, Velocity, apply_movement,
};
use bevy::ecs::schedule::Schedule;
use bevy::ecs::world::World;
use bevy::prelude::*;
use bevy::time::Fixed;
use std::sync::Once;
use std::time::Duration;
use tracing_subscriber::fmt;

static INIT: Once = Once::new();

/// Initializes global tracing and logging for the application.
///
/// Sets up a tracing subscriber with INFO-level formatting and installs a log tracer to route standard log messages through the tracing system. Ensures that initialization occurs only once, even if called multiple times.
pub fn init_tracing() {
    INIT.call_once(|| {
        // Install the LogTracer to convert logs from `log` crate to `tracing`
        tracing_log::LogTracer::init().expect("Failed to install LogTracer");

        // Build the subscriber with desired configuration
        let subscriber = fmt().with_max_level(tracing::Level::INFO).finish();

        // Set this subscriber as the global default
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");
    });
}

/// Runs all physics and particle system tests sequentially to ensure correct behavior.
///
/// This function initializes tracing and executes all test cases in a specific order to verify forward vector calculation, movement clamping, and engine particle spawning.
fn run_all_tests_in_order() {
    init_tracing();

    test_forward_vector_calculation();
    test_apply_movement_clamp();
    test_engine_particle_spawn();
}

/// Verifies that the forward vector calculation from a zero rotation produces the expected unit vector along the Y axis.
fn test_forward_vector_calculation() {
    init_tracing();

    let rotation = PhysicalRotation(0.0);
    let forward = Vec2::new(-rotation.0.sin(), rotation.0.cos());
    info!("Testing forward vector calculation:");
    info!("  Rotation: {}", rotation.0);
    info!("  Forward vector: {:?}", forward);
    info!("  Expected: {:?}", Vec2::new(0.0, 1.0));
    assert_eq!(forward, Vec2::new(0.0, 1.0));
}

/// Verifies that applying thrust to an entity accelerates it and clamps its velocity to the maximum allowed speed.
///
/// This test sets up a Bevy ECS world with an entity representing a ship, applies movement input, advances the simulation by one second, runs the movement system, and asserts that the resulting velocity does not exceed the defined maximum velocity constant.
///
/// # Examples
///
/// ```
/// test_apply_movement_clamp();
/// // Passes if the entity's velocity is clamped to MAX_VELOCITY
/// ```
fn test_apply_movement_clamp() {
    init_tracing();

    let mut world = World::new();
    world.spawn((
        MovementInputAccumulator { value: Vec2::Y },
        PhysicalRotation(0.0),
        Velocity(Vec3::ZERO),
    ));
    world.insert_resource(Time::<Fixed>::default());
    {
        let mut time = world.resource_mut::<Time<Fixed>>();
        time.advance_by(Duration::from_secs_f32(1.0));
    }

    let mut schedule = Schedule::default();
    schedule.add_systems(apply_movement);
    schedule.run(&mut world);

    let velocity = world.query::<&Velocity>().single(&world).0;
    let expected = MAX_VELOCITY;
    info!("Velocity after apply_movement: {}", velocity.y);
    assert!((velocity.y - expected).abs() < f32::EPSILON);
}

/// Verifies that the engine particle system spawns particles behind the ship when thrust input is applied.
///
/// This test sets up a Bevy ECS world with a ship entity receiving forward thrust, runs the engine particle system,
/// and asserts that a particle is spawned at the expected offset behind the ship.
///
/// # Examples
///
/// ```
/// test_engine_particle_spawn();
/// // Passes if the particle spawns at (0.0, -20.0) relative to the ship.
/// ```
fn test_engine_particle_spawn() {
    init_tracing();

    let mut world = World::new();
    world.spawn((
        MovementInputAccumulator { value: Vec2::Y },
        Transform::default(),
        PhysicalRotation(0.0),
    ));
    world.insert_resource(Assets::<Mesh>::default());
    world.insert_resource(Assets::<ColorMaterial>::default());

    let mut schedule = Schedule::default();
    schedule.add_systems(engine_particle_system);
    schedule.run(&mut world);

    let mut query = world.query::<(&Particle, &Transform)>();
    let (_, transform) = query.single(&world);
    assert_eq!(transform.translation.truncate(), Vec2::new(0.0, -20.0));
}
