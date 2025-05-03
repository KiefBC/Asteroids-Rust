use bevy::{prelude::*, sprite::Wireframe2dPlugin};
use asteroids_rust::GamePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin::default(), GamePlugin))
        .run();
}
