use bevy::prelude::*;
use bevy_inspector_egui::prelude::InspectorOptions;

#[derive(Default, Clone, Debug)]
pub struct Coords {
    pub x: u16,
    pub y: u16,
}

#[derive(Component)]
pub struct TileBase {
    pub x: u16,
    pub y: u16,
    pub tilemap_id: Entity,
}

impl TileBase {
    pub fn coords(&self) -> Coords {
        Coords {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Component)]
pub struct Atmos {
    pub p: f32,
    pub t: f32,
    pub n: f32,
}

#[derive(Component)]
pub struct Wall;

#[derive(Bundle, InspectorOptions)]
pub struct AtmosTile {
    pub base: TileBase,
    pub atmos: Atmos,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

#[derive(Bundle, InspectorOptions)]
pub struct WallTile {
    pub base: TileBase,
    pub wall: Wall,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

pub enum Tiles {
    Atmos(AtmosTile),
    Wall(WallTile),
}
