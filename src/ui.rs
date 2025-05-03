use bevy::prelude::*;

/// Spawn a bit of UI text to explain how to move the player.
pub fn spawn_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Press space to toggle wireframes"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
        .with_child((
            Text::new("Move the player with WASD"),
            TextFont {
                font_size: 25.0,
                ..default()
            },
        ));
}

pub fn toggle_wireframe(
    mut wireframe_config: ResMut<bevy::sprite::Wireframe2dConfig>, 
    keyboard: Res<ButtonInput<KeyCode>>, 
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}