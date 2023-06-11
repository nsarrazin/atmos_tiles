use std::time::Duration;

use bevy::prelude::*;
mod tilemap;

use tilemap::TileMapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(TileMapPlugin)
        .add_startup_system(setup)
        .run()
}

#[derive(Resource)]
struct GameConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(GameConfig {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    });

    // camera stuff
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        projection: OrthographicProjection {
            scale: 4.,
            far: 1000.,
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Component)]
pub struct AtmosTile {
    pub p: f32, // pressure
    pub t: f32, // temperature
    pub n: f32, // moles
    pub x: u16,
    pub y: u16,
}
