use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::item::taker;

use super::{ChunkedGrid, CHUNK_SIZE};

#[derive(Resource)]
pub struct WorldChunkedGrid {
    pub grid: ChunkedGrid,
    pub element_size: f32,
    pub grid_pointer_direction: taker::CardinalDirection,
    pub grid_mouse_position: Option<IVec2>
}

impl WorldChunkedGrid {
	fn new(element_size: f32) -> Self {
		Self { grid: ChunkedGrid::default(), element_size, grid_mouse_position: None, grid_pointer_direction: taker::CardinalDirection::Down }
	}

    pub fn grid_to_world_pos(&self, pos: Vec2) -> Vec2 {
        let mut vec: Vec2 = (pos + Vec2::splat(0.5)) * self.element_size;
        vec.y *= -1.;
        //println!("{} {}", pos, vec);
        //println!("GTW In: {} Out: {}", pos, vec);
        vec
    }

    pub fn world_pos_to_grid(&self, pos: Vec2) -> IVec2 {
        let mut convert = (pos / self.element_size).as_ivec2();
        if pos.x < 0. { convert.x -= 1; };
        if pos.y > 0. { convert.y += 1; };
        convert.y *= -1;
//        println!("WTG In: {} Out: {}", pos, convert);
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
	mut world_grid: ResMut<WorldChunkedGrid>,
    mut q_camera: Query<(&Camera, &GlobalTransform)>,
) {
	let window: &Window = q_windows.single();
    let (camera, camera_transform) = q_camera.single();
    if let Some(wp) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin)
    {
//        println!("Mouse pos: {}", wp);
        //let unit_size = window.height() / 8.;
        let world_position = wp;
        world_grid.grid_mouse_position = Some(world_grid.world_pos_to_grid(world_position.xz()));
    } else {
		world_grid.grid_mouse_position = None;
    }
}
