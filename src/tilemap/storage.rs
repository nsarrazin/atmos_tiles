use bevy::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct Coords {
    pub x: u64,
    pub y: u64,
}

#[derive(Component, Default, Debug, Clone)]
pub struct TileStorage {
    tiles: Vec<Option<Entity>>,
    pub size: Coords,
}

impl TileStorage {
    pub fn empty(size: Coords) -> Self {
        Self {
            tiles: vec![None; (size.x * size.y) as usize],
            size,
        }
    }

    pub fn get(&self, pos: Coords) -> Option<Entity> {
        let idx = (pos.y * self.size.x) + pos.x;
        return self.tiles[idx as usize];
    }

    pub fn set(&mut self, pos: Coords, tile: Entity) {
        let idx = (pos.y * self.size.x) + pos.x;
        println!("idx: {}", idx);
        println!("pos: {:?}", pos);
        self.tiles[idx as usize].replace(tile);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<Entity>> {
        return self.tiles.iter();
    }

    pub fn within_bounds(&self, pos: Coords) -> bool {
        if pos.x >= self.size.x || pos.y >= self.size.y {
            return false;
        } else {
            return true;
        }
    }
}
