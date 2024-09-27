use bevy::prelude::*;

use super::GridShape;

pub struct GridSquare {
	pub tl_position: IVec2,
	pub size: u32,
}

impl GridShape for GridSquare {
	fn get_grid_coordinates(&self) -> Vec<IVec2> {
		let mut res: Vec<IVec2> = Vec::new();
		for x in 0..self.size {
			for y in 0..self.size {
				res.push(self.tl_position + UVec2::new(x, y).as_ivec2());
			}
		}
		return res;
	}
}

#[derive(Component)]
pub struct GridEntity
{
	pub shape: GridSquare
}
