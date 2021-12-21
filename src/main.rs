use bevy::prelude::*;

mod plugins;
use plugins::{GamefieldPlugin, TilePlugin};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Ten Top Tiles".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamefieldPlugin)
        .add_plugin(TilePlugin)
        .run();
}
