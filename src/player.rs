use crate::physics;
// use avian2d::prelude::*;
use bevy::prelude::*;

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
    let ship_mesh = meshes.add(Triangle2d::new(
        nose_point - center_point,
        bottom_left_point - center_point,
        bottom_right_point - center_point,
    ));
    let ship_color = Color::srgb(0.0, 0.0, 1.0);

    // let vertices = vec![
    //     nose_point - center_point,         // Vertex 0
    //     bottom_left_point - center_point,  // Vertex 1
    //     bottom_right_point - center_point, // Vertex 2
    // ];
    // let indices = vec![[0, 1, 2]]; // One triangle
    // let ship_collider = Collider::triangle(
    //     (nose_point - center_point),
    //     (bottom_left_point - center_point),
    //     (bottom_right_point - center_point),
    // );

    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Name::new("Player"),
        ColorMesh2dBundle {
            mesh: ship_mesh.into(),
            material: materials.add(ship_color),
            transform: Transform::from_scale(Vec3::splat(0.4)),
            ..default()
        },
        physics::ShipPhysicsBundle::default(),
        // RigidBody::Dynamic, // Avian2D component
        // ship_collider, // Avian2D component
    ));
}
