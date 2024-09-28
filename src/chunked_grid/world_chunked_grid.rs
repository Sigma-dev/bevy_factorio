use bevy::{prelude::*, window::PrimaryWindow};

use super::{ChunkedGrid, CHUNK_SIZE};

#[derive(Resource)]
pub struct WorldChunkedGrid {
    pub grid: ChunkedGrid,
    pub element_size: f32,
    pub grid_mouse_position: Option<IVec2>
}

impl WorldChunkedGrid {
	fn new(element_size: f32) -> Self {
		Self { grid: ChunkedGrid::default(), element_size, grid_mouse_position: None }
	}

    pub fn grid_to_world_pos(&self, pos: IVec2) -> Vec2 {
        let mut vec: Vec2 = (pos.as_vec2() + Vec2::splat(0.5))  * self.element_size;
        vec
    }

    pub fn world_pos_to_grid(&self, pos: Vec2) -> IVec2 {
        let mut convert = (pos / self.element_size).as_ivec2();
        if pos.x < 0. { convert.x -= 1; };
        if pos.y < 0. { convert.y -= 1; };
        convert
    }
}

pub struct WorldChunkedGridPlugin {
    pub(crate) element_size: f32,
}

impl Plugin for WorldChunkedGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldChunkedGrid::new(self.element_size))
		.add_systems(Update, update_mouse_position);
    }
}

fn update_mouse_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
	mut world_grid: ResMut<WorldChunkedGrid>
) {
	let window: &Window = q_windows.single();
    if let Some(position) = window.cursor_position() {
		let position_from_start = position - window.size() / 2.;
        let unit_size = window.height() / 8.;
        let world_position = position_from_start / unit_size;
        world_grid.grid_mouse_position = Some(world_grid.world_pos_to_grid(world_position));
    } else {
		world_grid.grid_mouse_position = None;
    }
}
