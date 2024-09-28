use bevy::{prelude::*, window::PrimaryWindow};

use super::CHUNK_SIZE;

pub struct ChunkGridMousePositionPlugin {
	pub scale: f32,
}

#[derive(Resource)]
pub struct ChunkGridMousePosition {
	pub chunk_position: Option<IVec2>,
	pub grid_position: Option<IVec2>,
	scale: f32,
}

impl ChunkGridMousePosition {
	fn new(scale: f32) -> Self {
		Self { chunk_position: None, grid_position: None, scale }
	}
}

impl Plugin for ChunkGridMousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkGridMousePosition::new(self.scale))
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