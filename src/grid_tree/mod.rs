use core::{panic};

use bevy::{prelude::*};

pub use visualizer::*;
pub use grid_entity::*;
pub mod visualizer;
pub mod grid_entity;

#[derive(Resource, PartialEq, Clone, Debug)]
pub struct GridTreeChunk {
    chunks: Vec<TreeChunk>,
    size: u32,
    position: IVec2
}

#[derive(Clone, Debug, PartialEq)]
enum TreeChunk {
    Chunk(GridTreeChunk),
    Empty,
    Grid(Entity)
}

#[derive(Copy, Clone, Debug)]
enum ChunkOrder {
    TopLeft = 0,
    TopRight = 1,
    BotLeft = 2,
    BotRight = 3
}

trait GridShape {
	fn get_grid_coordinates(&self) -> Vec<IVec2>;
}

impl Default for GridTreeChunk {
    fn default() -> Self {
        Self {
            chunks: vec![TreeChunk::Empty; 4],
            size: 512,
            position: IVec2::ZERO
        }
    } 
}

impl GridTreeChunk {
    fn new(size: u32, pos: IVec2) -> Self {
        Self {
            chunks: vec![TreeChunk::Empty; 4],
            size: size,
            position: pos
        }
    }

    pub fn store_grid_position(&mut self, pos: IVec2) {
        if self.size == 1 {
            return;
        }
        let tree_chunk = self.get_chunk_at(pos);
        match tree_chunk {
            TreeChunk::Chunk(grid_tree_chunk) => grid_tree_chunk.store_grid_position(pos),
            TreeChunk::Empty => { panic!() },
            TreeChunk::Grid(_) => {},
        }
    }

    pub fn get_entity_at(&mut self, pos: IVec2) {
        let tree_chunk = self.get_chunk_at(pos);
    }

    pub fn insert(&mut self, shape: &impl GridShape) {
        for coor in shape.get_grid_coordinates() {
            self.store_grid_position(coor);
        }
    }
    fn get_chunk_at(&mut self, pos: IVec2) -> &mut TreeChunk {
        if pos.y >= self.position.y {
            if pos.x >= self.position.x {
                self.get_or_create(ChunkOrder::TopRight)
            } else {
                self.get_or_create(ChunkOrder::TopLeft)
            }
        } else {
            if pos.x >= self.position.x {
                self.get_or_create(ChunkOrder::BotRight)
            } else {
                self.get_or_create(ChunkOrder::BotLeft)
            }
        }
    }

    fn get_or_create(&mut self, chunk: ChunkOrder) -> &mut TreeChunk {
        println!("chunk: {chunk:?}");
        let size = self.size as i32;
        let new_pos = match chunk {
            ChunkOrder::TopLeft => self.position + IVec2::new(-size / 4, size / 4),
            ChunkOrder::TopRight => self.position + IVec2::new(size / 4, size / 4),
            ChunkOrder::BotLeft => self.position + IVec2::new(-size / 4, -size / 4),
            ChunkOrder::BotRight => self.position + IVec2::new(size / 4, -size / 4),
        };
        if  self.chunks[chunk as usize] == TreeChunk::Empty {
            self.chunks[chunk as usize] = TreeChunk::Chunk(GridTreeChunk::new(self.size / 2, new_pos));
        }
        return &mut self.chunks[chunk as usize];
    }

    fn is_empty(&self) -> bool {
        self.chunks == vec![TreeChunk::Empty, TreeChunk::Empty, TreeChunk::Empty, TreeChunk::Empty]
    }
}