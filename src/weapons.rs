use bevy::prelude::*;
use avian2d::prelude::*;
use crate::player::Name;

#[derive(Resource)]
pub struct ShootTimer(pub Timer);

#[derive(Resource)]
pub struct ShootCooldown {
    pub cooldown_seconds: f32,
    pub timer: Timer,
}

impl Default for ShootCooldown {
    fn default() -> Self {
        Self {
            cooldown_seconds: 0.2,
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct Bullet {
    pub lifetime: Timer,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}

pub fn shoot_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shoot_cooldown: ResMut<ShootCooldown>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, (With<Name>, Without<Bullet>)>,
) {
    shoot_cooldown.timer.tick(time.delta());
    
    if keyboard_input.pressed(KeyCode::Space) && shoot_cooldown.timer.finished() {
        if let Ok(player_transform) = player_query.get_single() {
            spawn_bullet(&mut commands, &mut meshes, &mut materials, player_transform);
            shoot_cooldown.timer = Timer::from_seconds(shoot_cooldown.cooldown_seconds, TimerMode::Once);
        }
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_transform: &Transform,
) {
    let bullet_speed = 400.0;
    let bullet_radius = 3.0;
    
    let forward = player_transform.rotation * Vec3::Y;
    let spawn_offset = forward * 40.0;
    let spawn_position = player_transform.translation + spawn_offset;
    
    let velocity = Vec2::new(forward.x, forward.y) * bullet_speed;
    
    let bullet_mesh = meshes.add(Circle::new(bullet_radius));
    let bullet_material = materials.add(Color::srgb(1.0, 1.0, 0.0));
    
    commands.spawn((
        Bullet::default(),
        ColorMesh2dBundle {
            mesh: bullet_mesh.into(),
            material: bullet_material,
            transform: Transform::from_translation(spawn_position),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::circle(bullet_radius),
        LinearVelocity(velocity),
    ));
}

pub fn bullet_lifetime_system(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
) {
    for (entity, mut bullet) in bullets.iter_mut() {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
