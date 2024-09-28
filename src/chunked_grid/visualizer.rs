use bevy::prelude::*;

use super::world_chunked_grid::WorldChunkedGrid;

pub struct ChunkedTreeVisualizerPlugin;

impl Plugin for ChunkedTreeVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_grid);
    }
}

fn draw_grid(
    world_grid: Res<WorldChunkedGrid>,
    mut gizmos: Gizmos
) {
    //gizmos.rect(world_grid.get_grid_world_pos(IVec2::ZERO), 0., Vec2::splat(world_grid.get_grid_world_size()), Color::srgb(0., 1., 0.));
   // draw_rect(&mut gizmos, world_grid.get_grid_world_pos(IVec2::ZERO), world_grid.element_size, Color::srgb(0., 1., 0.));
    for (pos, _) in world_grid.grid.chunks.iter() {
        draw_chunk(&mut gizmos, *pos, &world_grid);
    }
}

fn draw_chunk(
    gizmos: &mut Gizmos,
    pos: IVec2,
    world_grid: &Res<WorldChunkedGrid>,
) {
   // gizmos.rect(world_grid.get_grid_world_pos(pos), 0., Vec2::splat(world_grid.get_grid_world_size()), Color::srgb(1., 0., 0.));
   draw_rect(gizmos, world_grid.grid_to_world_pos(pos), world_grid.element_size, Color::srgb(1., 0., 0.));
}

fn draw_rect(gizmos: &mut Gizmos, pos: Vec2, size: f32, color: impl Into<Color>) {
  // gizmos.rect_2d(pos, 0., Vec2::splat(size), color);
   let pos_3d = Vec3::new(pos.x, 0., pos.y);
 //  gizmos.rect(pos_3d, Quat::IDENTITY, Vec2::splat(size), color);
}