use bevy::{prelude::*, utils::HashMap};
use grid_chunk::GridChunk;

use crate::grid_shape::GridShape;

pub(crate) const CHUNK_SIZE: usize = 32;

pub mod grid_chunk;
pub mod visualizer;
pub mod grid_mouse_position;

#[derive(Resource, Default)]
pub struct ChunkedGrid {
    chunks: HashMap<IVec2, GridChunk>,
}

impl ChunkedGrid {
    pub fn insert(&mut self, pos: IVec2, entity: Entity) {
        let chunk_pos = self.get_chunk_pos(pos);
        let maybe_chunk = self.get_chunk_mut(chunk_pos);
        let relative_pos =  (pos % 32).abs();
        if let Some(chunk) = maybe_chunk {
            chunk.insert(relative_pos, entity)
        } else {
            let mut grid = GridChunk::default();
            println!("DADA2 {} {}", pos, relative_pos);
            grid.insert(relative_pos, entity);
            self.chunks.insert(chunk_pos, grid);
        }
    }

    pub fn get_chunk_pos(&self, pos: IVec2) -> IVec2 {
        let mut chunk_pos = pos / 32;
        if (pos.x < 0) {
            chunk_pos.x -= 1;
        } 
        if (pos.y < 0) {
            chunk_pos.y -= 1;
        } 
        chunk_pos
    }
    
    pub fn get_entity_at(&self, pos: IVec2) -> Option<Entity> {
        let maybe_chunk = self.get_chunk(pos);
        
        maybe_chunk.map(| chunk | chunk.get(pos)).flatten()
    }

    fn get_chunk(&self, pos: IVec2) -> Option<&GridChunk> {
        self.chunks.get(&pos)
    }

    fn get_chunk_mut(&mut self, pos: IVec2) -> Option<&mut GridChunk> {
        self.chunks.get_mut(&pos)
    }

    pub fn insert_shape(&mut self, shape: &impl GridShape, entity: Entity,) {
        for coor in shape.get_grid_coordinates() {
            self.insert(coor, entity);
        }
    }
}
