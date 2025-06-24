use bevy::prelude::*;

/// Spawn a bit of UI text to explain how to move the player.
pub fn spawn_text(mut commands: Commands) {
    commands.spawn(TextBundle {
        text: Text::from_section(
            "Press space to toggle wireframes",
            TextStyle {
                font_size: 30.0,
                ..default()
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(TextBundle {
        text: Text::from_section(
            "Move the player with WASD || Press R to Reset Ship Location",
            TextStyle {
                font_size: 25.0,
                ..default()
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        ..default()
    });
}

/// Toggle the wireframe display when the spacebar is pressed.
pub fn toggle_wireframe(
    mut wireframe_config: ResMut<bevy::sprite::Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
