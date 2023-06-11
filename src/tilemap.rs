use bevy::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::GameConfig;

const TILE_SIZE: f32 = 128.;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_tilemap);
        // app.add_system(rotate_tilemaps);
    }
}

fn create_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("sprites/tilesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        27,
        27,
        Some(Vec2::new(0., 0.)),
        None,
    );

    let texture_atlas_handle = atlases.add(texture_atlas);

    let file = File::open("assets/maps/map1.txt").expect("No map file found");

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let idx = match char {
                    '0' => 0,
                    '1' => 11,
                    '2' => 22,
                    '3' => 30,
                    '4' => 33,
                    _ => 0,
                };

                commands.spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: idx,
                        ..default()
                    },
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * TILE_SIZE,
                        y as f32 * -TILE_SIZE,
                        0.,
                    )),
                    ..Default::default()
                });
            }
        }
    }
}

fn rotate_tilemaps(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite)>,
    mut config: ResMut<GameConfig>,
) {
    config.timer.tick(time.delta());
    for (mut transform, mut sprite) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(time.delta_seconds()));
    }
}
