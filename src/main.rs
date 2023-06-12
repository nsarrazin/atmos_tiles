use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod tilemap;
use tilemap::TileMapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TileMapPlugin)
        .add_startup_system(setup)
        .run()
}

pub fn setup(mut commands: Commands) {
    // camera stuff
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        projection: OrthographicProjection {
            scale: 4.,
            ..Default::default()
        },
        ..Default::default()
    });
}
