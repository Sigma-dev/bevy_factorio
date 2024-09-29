use bevy::prelude::*;

pub trait GridShape {
	fn get_grid_coordinates(&self) -> Vec<IVec2>;
	fn get_neighboring_coordinates(&self) -> Vec<IVec2>;
}