use asteroids_rust::physics::{PhysicalRotation, SHIP_SPEED};
use bevy::prelude::*;
use std::sync::Once;
use tracing_subscriber::fmt;

static INIT: Once = Once::new();

pub fn init_tracing() {
    INIT.call_once(|| {
        // Install the LogTracer to convert logs from `log` crate to `tracing`
        tracing_log::LogTracer::init().expect("Failed to install LogTracer");

        // Build the subscriber with desired configuration
        let subscriber = fmt()
            .with_max_level(tracing::Level::INFO)
            .finish();

        // Set this subscriber as the global default
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");
    });
}

/// The main test function that runs all other tests in order
#[test]
fn run_all_tests_in_order() {
    init_tracing();

    test_forward_vector_calculation();
    test_velocity_normalization();
}

/// Tests the calculation of the forward vector based on rotation
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

/// Tests the normalization of velocity to ensure consistent speed
fn test_velocity_normalization() {
    init_tracing();
    
    let input = Vec2::new(3.0, 4.0);
    let velocity = input.normalize_or_zero() * SHIP_SPEED;
    let speed_diff = (velocity.length() - SHIP_SPEED).abs();
    info!("Testing velocity normalization:");
    info!("  Input vector: {:?}", input);
    info!("  Normalized velocity: {:?}", velocity);
    info!("  Velocity length: {}", velocity.length());
    info!("  Expected length: {}", SHIP_SPEED);
    info!("  Difference: {}", speed_diff);
    assert!(speed_diff < f32::EPSILON);
}