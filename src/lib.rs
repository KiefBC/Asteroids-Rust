//! # Asteroids Rust
//! 
//! A simple Asteroids game implementation using the Bevy game engine.
//! 
//! This crate provides the core game functionality including:
//! - Physics simulation for ship movement
//! - Player controls and input handling
//! - UI elements and wireframe toggling
//! - Weapon systems
//! 
//! ## Game Structure
//! 
//! The game is structured as a Bevy plugin (`GamePlugin`) that can be added to a Bevy app.
//! It sets up all necessary systems and resources for the game to function.

/// Asteroids module containing asteroid entities, spawning, and collision systems
pub mod asteroids;
/// Particles module containing particle effects and explosion systems
pub mod particles;
/// Physics module containing movement, rotation, and collision components and systems
pub mod physics;
/// Player module containing player ship components and spawning systems
pub mod player;
/// UI module containing text display and wireframe toggle functionality
pub mod ui;
/// Weapons module containing shooting mechanics and timer resources
pub mod weapons;

use bevy::prelude::*;

/// Main plugin for the Asteroids game
/// 
/// This plugin sets up all game systems and resources when added to a Bevy app.
/// It handles the initialization of:
/// - Weapon timers
/// - Player spawning
/// - UI elements
/// - Physics systems
/// - Input handling
pub struct GamePlugin;

impl Plugin for GamePlugin {
    /// Builds the plugin by adding all necessary systems and resources to the app
    /// 
    /// # Arguments
    /// 
    /// * `app` - The Bevy app to add systems and resources to
    fn build(&self, app: &mut App) {
        app.insert_resource(weapons::ShootTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .insert_resource(weapons::ShootCooldown::default())
        .insert_resource(asteroids::AsteroidSpawnTimer::default())
        .insert_resource(asteroids::AsteroidCount::default())
        .add_systems(Startup, (ui::spawn_text, player::spawn_player))
        .add_systems(Update, (
            physics::reset_ship_position, 
            physics::wrap_screen_position, 
            ui::toggle_wireframe,
            weapons::shoot_system,
            weapons::bullet_lifetime_system,
            asteroids::spawn_asteroid_system,
            asteroids::wrap_asteroids,
            asteroids::bullet_asteroid_collision_system,
            particles::update_particles,
<<<<<<< HEAD
=======
            particles::engine_particle_system,
>>>>>>> b60c61a (engine particle effects, shooting physics, asteroid explosion, wrap around ship movement)
        ))
        .add_systems(FixedUpdate, physics::update_physics_state)
        .add_systems(
            PreUpdate,
            (
                physics::gather_movement_input,
                physics::apply_movement,
                physics::apply_rotation_input,
            ),
        )
        .add_systems(PostUpdate, physics::interpolate_rendered_transform);
    }
}