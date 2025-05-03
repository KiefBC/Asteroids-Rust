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
        .add_systems(Update, ui::toggle_wireframe)
        .add_systems(FixedUpdate, physics::advance_physics)
        .add_systems(
            RunFixedMainLoop,
            (
                physics::handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                physics::handle_rotation.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                physics::interpolate_rendered_transform
                    .in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        );
    }
}
