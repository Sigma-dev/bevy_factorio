use bevy::{prelude::*, utils::HashMap};

use crate::grid_shape::GridShape;

const CHUNK_SIZE: usize = 32;

pub mod grid_chunk;
pub mod visualizer;

#[derive(Resource, Default)]
pub struct WorldChunkedGrid {
    grid: ChunkedGrid,
    scale: f32,
}

impl ChunkGridMousePosition {
	fn new(scale: f32) -> Self {
		Self { chunk_position: None, grid_position: None, scale }
	}
}

impl Plugin for ChunkGridMousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldChunkedGrid::new(self.scale))
		.add_systems(Update, update_mouse_position);
    }
}

fn update_mouse_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
	mut grid_mouse_position: ResMut<ChunkGridMousePosition>
) {
	let window: &Window = q_windows.single();
    if let Some(position) = window.cursor_position() {
		let mut pos_from_middle = position - window.size() / 2.;
		pos_from_middle.y *= -1.;
		grid_mouse_position.chunk_position = Some((pos_from_middle * grid_mouse_position.scale).as_ivec2() / CHUNK_SIZE as i32);
		grid_mouse_position.grid_position = Some((pos_from_middle * grid_mouse_position.scale).as_ivec2());
    } else {
		grid_mouse_position.grid_position = None;
    }
}
