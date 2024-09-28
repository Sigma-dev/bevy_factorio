use bevy::{prelude::*, utils::HashMap};
use grid_chunk::GridChunk;

use crate::grid_shape::GridShape;

pub(crate) const CHUNK_SIZE: usize = 32;

pub mod grid_chunk;
pub mod visualizer;
pub mod world_chunked_grid;

#[derive(Resource, Default)]
pub struct ChunkedGrid {
    chunks: HashMap<IVec2, GridChunk>,
}

impl ChunkedGrid {
    pub fn insert(&mut self, pos: IVec2, entity: Entity) {
        let relative_pos =  self.grid_pos_to_chunk_grid(pos);
        let chunk_pos = self.get_chunk_pos(pos);
        let maybe_chunk = self.chunks.get_mut(&chunk_pos);
        if let Some(chunk) = maybe_chunk {
            chunk.insert(relative_pos, entity)
        } else {
            let mut chunk = GridChunk::default();
            chunk.insert(relative_pos, entity);
            self.chunks.insert(chunk_pos, chunk);
        }
    }

    pub fn get_chunk_pos(&self, pos: IVec2) -> IVec2 {
        let mut chunk_pos = pos / 32;
        if pos.x < 0 {
            chunk_pos.x -= 1;
        } 
        if pos.y < 0 {
            chunk_pos.y -= 1;
        } 
        chunk_pos
    }
    
    pub fn get_entity_at(&self, pos: IVec2) -> Option<Entity> {
        let maybe_chunk = self.get_chunk(pos);
        let chunk_grid = self.grid_pos_to_chunk_grid(pos);
        maybe_chunk.map(| chunk | chunk.get(chunk_grid)).flatten()
    }

    fn get_chunk(&self, grid_pos: IVec2) -> Option<&GridChunk> {
        self.chunks.get(&self.grid_pos_to_chunk_coordinates(grid_pos))
    }

    fn get_chunk_mut(&mut self, grid_pos: IVec2) -> Option<&mut GridChunk> {
        self.chunks.get_mut(&self.grid_pos_to_chunk_coordinates(grid_pos))
    }

    pub fn insert_shape(&mut self, shape: &impl GridShape, entity: Entity,) {
        for coor in shape.get_grid_coordinates() {
            self.insert(coor, entity);
        }
    }

    pub fn grid_pos_to_chunk_coordinates(&self, grid_pos: IVec2) -> IVec2 {
        let mut chunk_coor = grid_pos / CHUNK_SIZE as i32;
        if grid_pos.x < 0 { chunk_coor.x -= 1 }
        if grid_pos.y < 0 { chunk_coor.y -= 1 }
        chunk_coor
    }

    pub fn grid_pos_to_chunk_grid(&self, grid_pos: IVec2) -> IVec2 {
        let mut pos = grid_pos % CHUNK_SIZE as i32;
        if grid_pos.x < 0 { pos.x += CHUNK_SIZE as i32; }
        if grid_pos.y < 0 { pos.y += CHUNK_SIZE as i32; }
        pos 
    }
}
