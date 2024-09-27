use bevy::{math::VectorSpace, prelude::*};

use crate::grid_tree::TreeChunk;

use super::GridTreeChunk;

pub struct GridTreeVisualizerPlugin;

impl Plugin for GridTreeVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_tree);
    }
}

fn draw_tree(
    tree: Res<GridTreeChunk>,
    mut gizmos: Gizmos
) {
    for chunk in &tree.chunks {
        draw_chunk(&mut gizmos, &chunk);
    }
}

fn draw_chunk(
    gizmos: &mut Gizmos,
    tree_chunk: &TreeChunk
) {
    match tree_chunk {
        TreeChunk::Chunk(grid_tree_chunk) => {
            gizmos.rect_2d(grid_tree_chunk.position.as_vec2(), 0., Vec2::splat(grid_tree_chunk.size as f32), Color::srgb(1., 0., 0.));
            for ch in &grid_tree_chunk.chunks {
                draw_chunk(gizmos, &ch);
            }
        },
        TreeChunk::Empty => {},
        TreeChunk::Grid(_) => {},
    }
}