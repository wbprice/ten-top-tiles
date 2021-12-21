use bevy::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_tiles.system());
    }
}

struct Tile;

fn setup_tiles(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sprite_handle = materials.add(assets.load("tile.png").into());

    let mut tiles: Vec<Vec3> = vec![];
    for y in 0..6 {
        for x in 0..6 {
            tiles.push(Vec3::new(
                x as f32 * 48.0 + 24.0,
                y as f32 * 48.0 + 24.0,
                1.0,
            ));
        }
    }

    for translation in tiles {
        commands.spawn().insert(Tile).insert_bundle(SpriteBundle {
            material: sprite_handle.clone(),
            transform: Transform::from_translation(translation),
            sprite: Sprite::new(Vec2::splat(48.0)),
            ..Default::default()
        });
    }
}
