use asteroids_rust::GamePlugin;
use bevy::{prelude::*, sprite::Wireframe2dPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin::default(), GamePlugin))
        .run();
}
