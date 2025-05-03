use crate::physics;
use bevy::prelude::*;

/// Represents a player ship in the game
/// 
/// The Ship component is attached to the player entity and requires a Collider component.
#[derive(Component)]
#[require(Collider)]
pub struct Ship;

/// Provides a name for an entity
/// 
/// This component can be used to give a human-readable name to any entity in the game.
#[derive(Component)]
pub struct Name(pub String);

impl Name {
    /// Creates a new Name component with the given string
    pub fn new(name: &str) -> Self {
        Name(name.to_string())
    }
}

/// Marks an entity as collidable
/// 
/// Entities with this component can participate in collision detection.
#[derive(Component, Default)]
pub struct Collider;

/// Spawn the player sprite and a 2D camera.
/// 
/// It sets up the player's ship and camera in the game world.
pub fn spawn_player(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Center the mesh on its centroid so rotation pivots around the middle.
    let nose_point = Vec2::new(0.0, 66.666666);
    let bottom_left_point = Vec2::new(-50.0, -33.333332);
    let bottom_right_point = Vec2::new(50.0, -33.333332);
    let center_point = (nose_point + bottom_left_point + bottom_right_point) / 3.0;
    let ship = meshes.add(Triangle2d::new(
        nose_point - center_point,
        bottom_left_point - center_point,
        bottom_right_point - center_point,
    ));
    let ship_color = Color::srgb(0.0, 0.0, 1.0);

    commands.spawn(Camera2d);
    commands.spawn((
        Name::new("Player"),
        Mesh2d(ship),
        MeshMaterial2d(materials.add(ship_color)),
        Transform::from_scale(Vec3::splat(0.4)), // Scale the ship to fit the screen
        physics::ShipPhysicsBundle::default(),
        Ship,
        Collider,
    ));
}
