use bevy::{prelude::*, window::PrimaryWindow};

pub struct GridMousePositionPlugin {
	pub scale: f32,
}

#[derive(Resource)]
pub struct GridMousePosition {
	pub grid_position: Option<IVec2>,
	scale: f32,
}

impl GridMousePosition {
	fn new(scale: f32) -> Self {
		Self { grid_position: None, scale }
	}
}

impl Plugin for GridMousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GridMousePosition::new(self.scale))
		.add_systems(Update, update_mouse_position);
    }
}

fn update_mouse_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
	mut grid_mouse_position: ResMut<GridMousePosition>
) {
	let window: &Window = q_windows.single();
    if let Some(position) = window.cursor_position() {
		let mut pos_from_middle = position - window.size() / 2.;
		pos_from_middle.y *= -1.;
		grid_mouse_position.grid_position = Some((pos_from_middle * grid_mouse_position.scale).as_ivec2());
    } else {
		grid_mouse_position.grid_position = None;
    }
}