use core::{panic};
use std::process::exit;

use bevy::{prelude::*};

pub use grid_mouse_position::*;
pub use visualizer::*;
pub use grid_entity::*;
pub mod grid_mouse_position;
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

pub trait GridShape {
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

    pub fn get_entity_at(&mut self, pos: IVec2) -> Option<Entity> {
        let dir: ChunkOrder = self.determine_direction(pos);
        let mut chunk = &mut self.chunks[dir as usize];
        
        match chunk {
            TreeChunk::Chunk(grid_tree_chunk) => grid_tree_chunk.get_entity_at(pos),
            TreeChunk::Empty => None,
            TreeChunk::Grid(entity) => Some(*entity),
        }
    }

    pub fn insert(&mut self, entity: Entity, shape: &impl GridShape) {
        for coor in shape.get_grid_coordinates() {
            self.store_grid_position(coor, entity);
        }
    }

    fn determine_direction(&self, pos: IVec2) -> ChunkOrder{
        if pos.y >= self.position.y {
            if pos.x >= self.position.x {
                ChunkOrder::TopRight
            } else {
                ChunkOrder::TopLeft
            }
        } else {
            if pos.x >= self.position.x {
                ChunkOrder::BotRight
            } else {
                ChunkOrder::BotLeft
            }
        }
    }

    pub fn store_grid_position(&mut self, pos: IVec2, entity: Entity) {
        let dir: ChunkOrder = self.determine_direction(pos);
        let mut chunk = &mut self.chunks[dir as usize];

        match chunk {
            TreeChunk::Chunk(grid_tree_chunk) => grid_tree_chunk.store_grid_position(pos, entity),
            TreeChunk::Empty => {
                if let TreeChunk::Chunk(new_grid_tree_chunk) = self.init_chunk(dir, entity) {
                    new_grid_tree_chunk.store_grid_position(pos, entity);
                }
            }
            TreeChunk::Grid(_) => chunk = &mut TreeChunk::Grid(entity),
        }
    }

    fn init_chunk(&mut self, dir: ChunkOrder, entity: Entity) -> &mut TreeChunk {
        let new_size = (self.size / 2) as i32;
        println!("Size: {new_size}");
        if new_size == 0 {
            exit(30);
        }
        self.chunks[dir as usize] = 
        if new_size == 1 {
            TreeChunk::Grid(entity)
        } else {
            let offset = match dir {
                ChunkOrder::TopLeft => IVec2::new(-new_size, new_size),
                ChunkOrder::TopRight => IVec2::new(new_size, new_size),
                ChunkOrder::BotLeft => IVec2::new(-new_size, -new_size),
                ChunkOrder::BotRight => IVec2::new(new_size, -new_size),
            };
            TreeChunk::Chunk(GridTreeChunk::new(new_size as u32, self.position + offset / 2))
        };
        return &mut self.chunks[dir as usize];
    }

    fn is_empty(&self) -> bool {
        self.chunks == vec![TreeChunk::Empty, TreeChunk::Empty, TreeChunk::Empty, TreeChunk::Empty]
    }
}