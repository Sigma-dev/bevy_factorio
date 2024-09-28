use bevy::prelude::*;

pub trait GridShape {
	fn get_grid_coordinates(&self) -> Vec<IVec2>;
}