use bevy::prelude::*;

mod plugins;
use plugins::GamefieldPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamefieldPlugin)
        .run();
}
