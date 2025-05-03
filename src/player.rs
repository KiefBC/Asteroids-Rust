use bevy::prelude::*;

/// Since Bevy's default 2D camera setup is scaled such that
/// one unit is one pixel, you can think of this as
/// "How many pixels per second should the player move?"
pub const SHIP_SPEED: f32 = 500.;
/// Radians per Second
pub const ROTATION_SPEED: f32 = 3.;

#[derive(Component)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
#[require(Collider)]
pub struct Ship;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component, Default)]
pub struct Collider;

#[derive(Resource)]
pub struct ShootTimer(pub Timer);

/// Spawn the player sprite and a 2D camera.
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
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
        Name("Player".to_string()),
        Mesh2d(ship),
        MeshMaterial2d(materials.add(ship_color)),
        Transform::from_scale(Vec3::splat(0.3)),
        super::physics::AccumulatedInput::default(),
        super::physics::Velocity::default(),
        super::physics::PhysicalTranslation::default(),
        super::physics::PreviousPhysicalTranslation::default(),
        super::physics::PhysicalRotation::default(),
        super::physics::PreviousPhysicalRotation::default(),
        Ship,
        Collider,
    ));
}