use bevy::prelude::*;

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
}