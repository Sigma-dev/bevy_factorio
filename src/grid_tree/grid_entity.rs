use bevy::prelude::*;

use crate::grid_shape::GridShape;

#[derive(Clone)]
pub struct GridSquare {
	pub bl_position: IVec2,
	pub size: u32,
}

impl GridShape for GridSquare {
	fn get_grid_coordinates(&self) -> Vec<IVec2> {
		let mut res: Vec<IVec2> = Vec::new();
		for x in 0..self.size {
			for y in 0..self.size {
				res.push(self.bl_position + UVec2::new(x, y).as_ivec2());
			}
		}
		return res;
	}

	fn get_neighboring_coordinates(&self) -> Vec<IVec2> {
		let mut res: Vec<IVec2> = Vec::new();
		for x in 0..(self.size as i32) {
			res.push(self.bl_position + IVec2::new(x, self.size as i32));
		}
		for y in 0..(self.size as i32) {
			res.push(self.bl_position + IVec2::new(-1, y));
		}
		for y in 0..(self.size as i32) {
			res.push(self.bl_position + IVec2::new(self.size as i32, y));
		}
		for x in 0..(self.size as i32) {
			res.push(self.bl_position + IVec2::new(x, -1));
		}
		res
	}
}

#[derive(Component)]
pub struct GridEntity
{
	pub shape: GridSquare,
	pub grid_position: IVec2,
}