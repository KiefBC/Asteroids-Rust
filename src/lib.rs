pub mod physics;
pub mod player;
pub mod ui;
pub mod weapons;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(weapons::ShootTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, (ui::spawn_text, player::spawn_player))
        .add_systems(Update, (physics::reset_ship_position, ui::toggle_wireframe))
        .add_systems(FixedUpdate, physics::update_physics_state)
        .add_systems(
            RunFixedMainLoop,
            (
                physics::gather_movement_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                physics::apply_movement.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                physics::apply_rotation_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                physics::interpolate_rendered_transform
                    .in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        );
    }
}
