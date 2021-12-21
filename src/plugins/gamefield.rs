use bevy::prelude::*;

pub struct GamefieldPlugin;
impl Plugin for GamefieldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_dots.system());
    }
}

struct Dot;

pub fn setup_dots(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sprite_handle = materials.add(assets.load("white_dot.png").into());

    let mut grid: Vec<Vec3> = vec![];
    for y in 0..7 {
        for x in 0..7 {
            grid.push(Vec3::new(x as f32 * 48.0, y as f32 * 48.0, 1.0));
        }
    }

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    for translation in grid {
        commands.spawn().insert(Dot).insert_bundle(SpriteBundle {
            material: sprite_handle.clone(),
            transform: Transform::from_translation(translation),
            sprite: Sprite::new(Vec2::splat(16.0)),
            ..Default::default()
        });
    }
}
