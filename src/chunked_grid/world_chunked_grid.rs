use bevy::{prelude::*, window::PrimaryWindow};

use super::{ChunkedGrid, CHUNK_SIZE};

#[derive(Resource)]
pub struct WorldChunkedGrid {
    pub grid: ChunkedGrid,
    pub scale: f32,
    pub grid_mouse_position: Option<IVec2>
}

impl WorldChunkedGrid {
	fn new(scale: f32) -> Self {
		Self { grid: ChunkedGrid::default(), scale, grid_mouse_position: None }
	}

    pub fn get_grid_world_pos(&self, pos: IVec2) -> Vec2 {
        (pos.as_vec2() + Vec2::splat(0.5))  * self.get_grid_world_size()
    }

    pub fn get_grid_world_size(&self) -> f32 {
        CHUNK_SIZE as f32 * self.scale
    }
}

pub struct WorldChunkedGridPlugin {
    pub(crate) scale: f32,
}

impl Plugin for WorldChunkedGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldChunkedGrid::new(self.scale))
		.add_systems(Update, update_mouse_position);
    }
}

fn update_mouse_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
	mut grid_mouse_position: ResMut<WorldChunkedGrid>
) {
	let window: &Window = q_windows.single();
    if let Some(position) = window.cursor_position() {
		let mut world_position = position - window.size() / 2.;
		world_position.y *= -1.;
		grid_mouse_position.grid_mouse_position = Some((world_position / grid_mouse_position.scale).as_ivec2());
    } else {
		grid_mouse_position.grid_mouse_position = None;
    }
}
