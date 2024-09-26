use bevy::{math::VectorSpace, prelude::*};

pub use visualizer::*;
pub mod visualizer;

#[derive(Resource, PartialEq)]
pub struct GridTreeChunk {
    chunks: Vec<Option<GridTreeChunk>>,
    size: u32,
    position: IVec2
}

#[derive(Copy, Clone)]
enum ChunkOrder {
    TopLeft = 0,
    TopRight = 1,
    BotLeft = 2,
    BotRight = 3
}

impl Default for GridTreeChunk {
    fn default() -> Self {
        Self {
            chunks: vec![None, None, None, None],
            size: 512,
            position: IVec2::ZERO
        }
    } 
}

impl GridTreeChunk {
    fn new(size: u32, pos: IVec2) -> Self {
        Self {
            chunks: vec![None, None, None, None],
            size: size,
            position: pos
        }
    }

    pub fn store_grid_position(&mut self, pos: IVec2) {
        if self.size == 1 {
            println!("Yes");
            return;
        }
        let chunk = self.get_chunk_at(pos);
        chunk.store_grid_position(pos);
    }

    fn get_chunk_at(&mut self, pos: IVec2) -> &mut GridTreeChunk {
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

    fn get_or_create(&mut self, chunk: ChunkOrder) -> &mut GridTreeChunk {
        let size = self.size as i32;
        let new_pos = match chunk {
            ChunkOrder::TopLeft => self.position + IVec2::new(-size / 4, size / 4),
            ChunkOrder::TopRight => self.position + IVec2::new(size / 4, size / 4),
            ChunkOrder::BotLeft => -self.position + IVec2::new(-size / 4, -size / 4),
            ChunkOrder::BotRight => self.position + IVec2::new(size / 4, -size / 4),
        };
        if  self.chunks[chunk as usize].is_none() {
            self.chunks[chunk as usize] = Some(GridTreeChunk::new(self.size / 2, new_pos));
        }
        return self.chunks[chunk as usize].as_mut().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.chunks == vec![None, None, None, None]
    }
}