use bevy::prelude::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::AtmosTile;
use crate::GameConfig;

mod storage;
pub use storage::*;

const TILE_SIZE: f32 = 128.;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_tilemap);
        app.add_system(rotate_over_time);
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

    let map_size = Coords { x: 12, y: 12 };

    let tile_storage_id = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile_pos = Coords {
                    x: x as u64,
                    y: y as u64,
                };

                if char == '0' {
                    continue;
                }

                let texture_idx: usize = match char {
                    '1' => 0,
                    '2' => 11,
                    '3' => 30,
                    '4' => 33,
                    _ => 0,
                };

                let tile = commands.spawn((
                    SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: texture_idx,
                            ..default()
                        },
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            x as f32 * TILE_SIZE,
                            y as f32 * -TILE_SIZE,
                            0.,
                        )),
                        ..Default::default()
                    },
                    AtmosTile {
                        p: 0.,
                        t: 0.,
                        n: 0.,
                        x: x as u16,
                        y: y as u16,
                        tilemap_id: tile_storage_id,
                    },
                ));
                tile_storage.set(tile_pos, tile.id());
            }
        }
    }

    commands.entity(tile_storage_id).insert(tile_storage);
}

fn rotate_over_time(
    time: Res<Time>,
    mut query_storage: Query<&mut TileStorage>,
    mut query: Query<&mut Transform>,
) {
    let seconds = time.elapsed_seconds() as u64;

    let tile_storage = query_storage.single_mut();

    let idx = seconds % tile_storage.size.x;

    let pos = Coords { x: idx, y: 0 };

    let mut tile = query.get_mut(tile_storage.get(pos).unwrap()).unwrap();

    tile.rotate(Quat::from_rotation_z(0.01));
}
