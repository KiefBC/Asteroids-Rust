use bevy::prelude::*;
use rand::prelude::*;

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