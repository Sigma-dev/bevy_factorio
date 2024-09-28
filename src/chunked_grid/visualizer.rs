use bevy::{gizmos::grid, math::VectorSpace, prelude::*};

use super::{grid_chunk::GridChunk, ChunkedGrid, CHUNK_SIZE};

pub struct ChunkedTreeVisualizerPlugin;

impl Plugin for ChunkedTreeVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_grid);
    }
}

fn draw_grid(
    grid: Res<ChunkedGrid>,
    mut gizmos: Gizmos
) {
    gizmos.rect_2d(Vec2::ZERO + Vec2::splat((CHUNK_SIZE / 2) as f32), 0., Vec2::splat(CHUNK_SIZE as f32), Color::srgb(0., 1., 0.));
    for (pos, chunk) in grid.chunks.iter() {
        draw_chunk(&mut gizmos, *pos, &chunk);
    }
}

fn draw_chunk(
    gizmos: &mut Gizmos,
    pos: IVec2,
    grid_chunk: &GridChunk
) {
    gizmos.rect_2d((pos * CHUNK_SIZE as i32).as_vec2() + Vec2::splat((CHUNK_SIZE / 2) as f32) , 0., Vec2::splat(CHUNK_SIZE as f32), Color::srgb(1., 0., 0.));
}