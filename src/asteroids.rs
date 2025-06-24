use bevy::prelude::*;
use avian2d::prelude::*;
use rand::prelude::*;
use crate::weapons::Bullet;
use crate::particles;

#[derive(Component)]
pub struct Asteroid {
    pub size: AsteroidSize,
}

#[derive(Clone, Copy, Debug)]
pub enum AsteroidSize {
    Large,
    Medium, 
    Small,
}

impl AsteroidSize {
    pub fn radius(self) -> f32 {
        match self {
            AsteroidSize::Large => 40.0,
            AsteroidSize::Medium => 25.0,
            AsteroidSize::Small => 15.0,
        }
    }
    
    pub fn split(self) -> Option<AsteroidSize> {
        match self {
            AsteroidSize::Large => Some(AsteroidSize::Medium),
            AsteroidSize::Medium => Some(AsteroidSize::Small),
            AsteroidSize::Small => None,
        }
    }
}

#[derive(Resource)]
pub struct AsteroidSpawnTimer(pub Timer);

impl Default for AsteroidSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct AsteroidCount {
    pub max_asteroids: usize,
    pub current_count: usize,
}

impl Default for AsteroidCount {
    fn default() -> Self {
        Self {
            max_asteroids: 8,
            current_count: 0,
        }
    }
}

pub fn spawn_asteroid_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_timer: ResMut<AsteroidSpawnTimer>,
    mut asteroid_count: ResMut<AsteroidCount>,
    time: Res<Time>,
    windows: Query<&Window>,
) {
    spawn_timer.0.tick(time.delta());
    
    if spawn_timer.0.finished() && asteroid_count.current_count < asteroid_count.max_asteroids {
        if let Ok(window) = windows.get_single() {
            spawn_asteroid_at_edge(&mut commands, &mut meshes, &mut materials, window, AsteroidSize::Large);
            asteroid_count.current_count += 1;
        }
    }
}

pub fn spawn_asteroid_at_edge(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    window: &Window,
    size: AsteroidSize,
) {
    let mut rng = thread_rng();
    let radius = size.radius();
    
    let (spawn_x, spawn_y) = {
        let margin = radius + 50.0;
        let width = window.width();
        let height = window.height();
        
        let edge = rng.gen_range(0..4);
        match edge {
            0 => (rng.gen_range(-width/2.0 - margin..width/2.0 + margin), height/2.0 + margin), // Top
            1 => (width/2.0 + margin, rng.gen_range(-height/2.0 - margin..height/2.0 + margin)), // Right
            2 => (rng.gen_range(-width/2.0 - margin..width/2.0 + margin), -height/2.0 - margin), // Bottom
            _ => (-width/2.0 - margin, rng.gen_range(-height/2.0 - margin..height/2.0 + margin)), // Left
        }
    };
    
    let velocity = Vec2::new(
        rng.gen_range(-100.0..100.0),
        rng.gen_range(-100.0..100.0),
    );
    
    let angular_velocity = rng.gen_range(-2.0..2.0);
    
    let asteroid_mesh = meshes.add(Circle::new(radius));
    let asteroid_material = materials.add(Color::srgb(0.7, 0.7, 0.7));
    
    commands.spawn((
        Asteroid { size },
        ColorMesh2dBundle {
            mesh: asteroid_mesh.into(),
            material: asteroid_material,
            transform: Transform::from_translation(Vec3::new(spawn_x, spawn_y, 0.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::circle(radius),
        LinearVelocity(velocity),
        AngularVelocity(angular_velocity),
    ));
}

pub fn wrap_asteroids(
    mut asteroids: Query<&mut Transform, With<Asteroid>>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.get_single() {
        let width = window.width();
        let height = window.height();
        let margin = 100.0;
        
        for mut transform in asteroids.iter_mut() {
            let pos = &mut transform.translation;
            
            if pos.x > width/2.0 + margin {
                pos.x = -width/2.0 - margin;
            } else if pos.x < -width/2.0 - margin {
                pos.x = width/2.0 + margin;
            }
            
            if pos.y > height/2.0 + margin {
                pos.y = -height/2.0 - margin;
            } else if pos.y < -height/2.0 - margin {
                pos.y = height/2.0 + margin;
            }
        }
    }
}

pub fn bullet_asteroid_collision_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut collision_events: EventReader<CollisionStarted>,
    bullets: Query<Entity, With<Bullet>>,
    asteroids: Query<(Entity, &Transform, &Asteroid)>,
    mut asteroid_count: ResMut<AsteroidCount>,
    windows: Query<&Window>,
) {
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        let (bullet_entity, asteroid_entity) = if bullets.contains(*entity1) && asteroids.contains(*entity2) {
            (*entity1, *entity2)
        } else if bullets.contains(*entity2) && asteroids.contains(*entity1) {
            (*entity2, *entity1)
        } else {
            continue;
        };
        
        if let Ok((_, transform, asteroid)) = asteroids.get(asteroid_entity) {
            let position = transform.translation.truncate();
            let size_radius = asteroid.size.radius();
            
            commands.entity(bullet_entity).despawn();
            commands.entity(asteroid_entity).despawn();
            asteroid_count.current_count -= 1;
            
            particles::spawn_asteroid_destruction_particles(
                &mut commands,
                &mut meshes,
                &mut materials,
                position,
                size_radius,
            );
            
            if let Some(smaller_size) = asteroid.size.split() {
                if let Ok(_window) = windows.get_single() {
                    for _ in 0..2 {
                        spawn_asteroid_fragment(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            position,
                            smaller_size,
                        );
                        asteroid_count.current_count += 1;
                    }
                }
            }
        }
    }
}

pub fn spawn_asteroid_fragment(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    size: AsteroidSize,
) {
    let mut rng = thread_rng();
    let radius = size.radius();
    
    let velocity = Vec2::new(
        rng.gen_range(-80.0..80.0),
        rng.gen_range(-80.0..80.0),
    );
    
    let angular_velocity = rng.gen_range(-3.0..3.0);
    
    let asteroid_mesh = meshes.add(Circle::new(radius));
    let asteroid_material = materials.add(Color::srgb(0.7, 0.7, 0.7));
    
    commands.spawn((
        Asteroid { size },
        ColorMesh2dBundle {
            mesh: asteroid_mesh.into(),
            material: asteroid_material,
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::circle(radius),
        LinearVelocity(velocity),
        AngularVelocity(angular_velocity),
    ));
}

pub fn despawn_asteroids(
    mut commands: Commands,
    asteroids: Query<Entity, With<Asteroid>>,
    mut asteroid_count: ResMut<AsteroidCount>,
) {
    for entity in asteroids.iter() {
        commands.entity(entity).despawn();
    }
    asteroid_count.current_count = 0;
}