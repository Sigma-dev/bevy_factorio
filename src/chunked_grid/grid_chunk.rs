use bevy::prelude::*;

use super::CHUNK_SIZE;

#[derive(Debug)]
pub(crate) struct GridChunk {
    grid: [[Option<Entity>; CHUNK_SIZE]; CHUNK_SIZE]
}

impl Default for GridChunk {
    fn default() -> Self {
        Self {
            grid: [[None; CHUNK_SIZE]; CHUNK_SIZE]
        }
    } 
}

impl GridChunk {
    pub fn insert(&mut self, pos: IVec2, entity: Entity) {
        self.grid[pos.y as usize][pos.x as usize] = Some(entity);
    }

    pub fn get(&self, pos: IVec2) -> Option<Entity> {
        self.grid[pos.y as usize][pos.x as usize]
    }
}