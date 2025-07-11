use bevy::prelude::*;
use rand::prelude::*;

use crate::physics::{InputAccumulator, MovementInputAccumulator, PhysicalRotation};

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub initial_size: f32,
    pub fade_rate: f32,
}

impl Particle {
    pub fn new(lifetime_seconds: f32, size: f32) -> Self {
        Self {
            lifetime: Timer::from_seconds(lifetime_seconds, TimerMode::Once),
            initial_size: size,
            fade_rate: 1.0 / lifetime_seconds,
        }
    }
}

#[derive(Component)]
pub struct ParticleVelocity {
    pub velocity: Vec2,
    pub drag: f32,
}

impl ParticleVelocity {
    pub fn new(velocity: Vec2, drag: f32) -> Self {
        Self { velocity, drag }
    }
}

pub fn spawn_explosion_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    particle_count: usize,
    base_color: Color,
) {
    let mut rng = thread_rng();
    
    for _ in 0..particle_count {
        let particle_size = rng.gen_range(1.0..4.0);
        let lifetime = rng.gen_range(0.5..1.5);
        
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..150.0);
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
        
        let color_variation = rng.gen_range(0.8..1.2);
        let particle_color = Color::srgba(
            (base_color.to_srgba().red * color_variation).clamp(0.0, 1.0),
            (base_color.to_srgba().green * color_variation).clamp(0.0, 1.0),
            (base_color.to_srgba().blue * color_variation).clamp(0.0, 1.0),
            1.0,
        );
        
        let particle_mesh = meshes.add(Circle::new(particle_size));
        let particle_material = materials.add(particle_color);
        
        let offset = Vec2::new(
            rng.gen_range(-5.0..5.0),
            rng.gen_range(-5.0..5.0),
        );
        
        commands.spawn((
            Particle::new(lifetime, particle_size),
            ParticleVelocity::new(velocity, 2.0),
            ColorMesh2dBundle {
                mesh: particle_mesh.into(),
                material: particle_material,
                transform: Transform::from_translation(Vec3::new(
                    position.x + offset.x,
                    position.y + offset.y,
                    0.1,
                )),
                ..default()
            },
        ));
    }
}

pub fn update_particles(
    mut commands: Commands,
    mut particles: Query<(Entity, &mut Particle, &mut ParticleVelocity, &mut Transform, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (entity, mut particle, mut particle_velocity, mut transform, material_handle) in particles.iter_mut() {
        particle.lifetime.tick(time.delta());
        
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        
        let drag = particle_velocity.drag;
        particle_velocity.velocity *= 1.0 - (drag * time.delta_seconds());
        
        transform.translation.x += particle_velocity.velocity.x * time.delta_seconds();
        transform.translation.y += particle_velocity.velocity.y * time.delta_seconds();
        
        let life_percent = particle.lifetime.elapsed_secs() / particle.lifetime.duration().as_secs_f32();
        let scale = (1.0 - life_percent * 0.5).max(0.1);
        let alpha = (1.0 - life_percent).max(0.0);
        
        transform.scale = Vec3::splat(scale);
        
        if let Some(material) = materials.get_mut(material_handle) {
            let current_color = material.color.to_srgba();
            material.color = Color::srgba(
                current_color.red,
                current_color.green,
                current_color.blue,
                alpha,
            );
        }
    }
}

/// Spawns explosion and spark particles at an asteroid's position to simulate its destruction.
///
/// The number and appearance of particles are scaled based on the asteroid's size, producing both orange/yellow explosion particles and bright spark particles at the specified position. This function is typically called when an asteroid is destroyed to create a visually impactful effect.
///
/// # Examples
///
/// ```
/// spawn_asteroid_destruction_particles(
///     &mut commands,
///     &mut meshes,
///     &mut materials,
///     Vec2::new(100.0, 200.0),
///     30.0,
/// );
/// ```
pub fn spawn_asteroid_destruction_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    asteroid_size: f32,
) {
    let particle_count = ((asteroid_size / 10.0) * 8.0) as usize;
    let base_color = Color::srgb(0.9, 0.6, 0.2); // Orange/yellow explosion color
    
    spawn_explosion_particles(
        commands,
        meshes,
        materials,
        position,
        particle_count,
        base_color,
    );
    
    let sparks_count = ((asteroid_size / 15.0) * 5.0) as usize;
    let spark_color = Color::srgb(1.0, 1.0, 0.8); // Bright sparks
    
    spawn_explosion_particles(
        commands,
        meshes,
        materials,
        position,
        sparks_count,
        spark_color,
    );
}

/// Spawns a single particle to simulate engine thrust at a given position and direction.
///
/// The particle has randomized size, lifetime, and velocity (opposite to the provided direction with slight variance), and uses a fixed orange color. Intended for use in engine exhaust effects.
///
/// # Examples
///
/// ```
/// // Spawns an engine thrust particle at (0.0, 0.0) moving left
/// spawn_engine_particle(&mut commands, &mut meshes, &mut materials, Vec2::ZERO, Vec2::NEG_X);
/// ```
pub fn spawn_engine_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    direction: Vec2,
) {
    let mut rng = thread_rng();

    let particle_size = rng.gen_range(1.0..3.0);
    let lifetime = rng.gen_range(0.2..0.4);

    // Particles travel opposite the thrust direction with slight variation
    let velocity_variance = Vec2::new(
        rng.gen_range(-0.2..0.2),
        rng.gen_range(-0.2..0.2),
    );
    let velocity = (direction + velocity_variance) * rng.gen_range(60.0..100.0);

    let particle_mesh = meshes.add(Circle::new(particle_size));
    let particle_material = materials.add(Color::srgb(1.0, 0.5, 0.2));

    commands.spawn((
        Particle::new(lifetime, particle_size),
        ParticleVelocity::new(velocity, 2.0),
        ColorMesh2dBundle {
            mesh: particle_mesh.into(),
            material: particle_material,
            transform: Transform::from_translation(position.extend(0.1)),
            ..default()
        },
    ));
}

/// Emits engine thrust particles for entities applying forward thrust.
///
/// For each entity with movement input, transform, and rotation, this system checks if forward thrust is active and spawns a particle effect behind the entity to simulate engine exhaust. The particle is emitted opposite to the entity's facing direction and offset from its position.
///
/// # Examples
///
/// ```
/// // Add the system to your Bevy app:
/// app.add_system(engine_particle_system);
/// ```
pub fn engine_particle_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&MovementInputAccumulator, &Transform, &PhysicalRotation)>,
) {
    for (input_acc, transform, rotation) in query.iter() {
        let input = input_acc.get();
        if input.y > 0.0 {
            let forward = Vec2::new(-rotation.0.sin(), rotation.0.cos());
            let spawn_pos = transform.translation.truncate() - forward * 20.0;
            spawn_engine_particle(
                &mut commands,
                &mut meshes,
                &mut materials,
                spawn_pos,
                -forward,
            );
        }
    }
}
