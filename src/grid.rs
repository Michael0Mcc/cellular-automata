pub mod wireworld;

use wireworld::Cell as WWCell;
use wireworld::{update_wireworld, get_color_wireworld};

use ggez::graphics;

#[derive(Debug, Copy, Clone)]
pub enum CellType {
	Empty,
	WireWorld(WWCell),
}

pub enum Direction {
	North,
	NorthEast,
	East,
	SouthEast,
	South,
	SouthWest,
	West,
	NorthWest,
}

pub struct Grid {
	cells: Vec<Vec<CellType>>
}

impl Grid {
	pub fn new(x_count: usize, y_count: usize, cell_type: CellType) -> Grid {
		Grid {
			cells: vec![vec![cell_type; x_count]; y_count],
		}
	}

	pub fn x_count(&self) -> usize {
		self.cells[0].len()
	}

	pub fn y_count(&self) -> usize {
		self.cells.len()
	}

	pub fn get_neighbor(&self, current_x: usize, current_y: usize, dir: Direction) -> CellType {
		match dir {
			Direction::North 	 => self.cells[current_y -1][current_x],
			Direction::NorthEast => self.cells[current_y -1][current_x +1],
			Direction::East 	 => self.cells[current_y][current_x +1],
			Direction::SouthEast => self.cells[current_y +1][current_x +1],
			Direction::South 	 => self.cells[current_y +1][current_x],
			Direction::SouthWest => self.cells[current_y +1][current_x -1],
			Direction::West 	 => self.cells[current_y][current_x -1],
			Direction::NorthWest => self.cells[current_y -1][current_x -1],
		}
	}

	pub fn get_cell(&self, x: usize, y: usize) -> CellType {
		self.cells[y][x]
	}

	pub fn set_cell(&mut self, cell_type: CellType, x: usize, y: usize) {
		self.cells[y][x] = cell_type;
	}

	pub fn get_color(&self, x: usize, y: usize) -> graphics::Color {
		match self.get_cell(x, y) {
			CellType::WireWorld(cell) => get_color_wireworld(cell, x, y),
			_ => graphics::Color::new(0.2, 0.2, 0.2, 1.0),
		}
	}

	pub fn update(&mut self) {
		let ref_grid: Grid = Grid {cells: self.cells.clone() };
		for y in 0..self.y_count() {
			for x in 0..self.x_count() {
				match ref_grid.get_cell(x, y) {
					CellType::WireWorld(cell) => update_wireworld(self, &ref_grid, cell, x, y),
					_ => (),
				}	
			}
		}
	}
}

