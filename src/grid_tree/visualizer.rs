use bevy::{math::VectorSpace, prelude::*};

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
    chunk_option: &Option<GridTreeChunk>
) {
    let Some(chunk) = chunk_option else {return;};
    println!("{}", chunk.position);
    gizmos.rect_2d(chunk.position.as_vec2(), 0., Vec2::splat(chunk.size as f32), Color::srgb(1., 0., 0.));
    for ch in &chunk.chunks {
        draw_chunk(gizmos, &ch);
    }
}