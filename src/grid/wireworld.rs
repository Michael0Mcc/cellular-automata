use crate::grid::*;
use ggez::graphics;

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Empty,
	ElectronHead,
	ElectronTail,
	Conductor,
}

pub fn update_wireworld(grid: &mut Grid, g: &Grid, cell: Cell, x: usize, y: usize) {
	match cell {
		WWCell::ElectronHead => grid.set_cell(CellType::WireWorld(Cell::ElectronTail), x, y),
		WWCell::ElectronTail => grid.set_cell(CellType::WireWorld(Cell::Conductor), x, y),
		WWCell::Conductor => {
			let mut count = 0;
			if y > 0 {
				if let CellType::WireWorld(c) = g.get_neighbor(x, y, Direction::North) {
					if let Cell::ElectronHead = c {
						count += 1;
					}
				}
				if x < grid.x_count() -1 {
					if let CellType::WireWorld(c) = g.get_neighbor(x, y, Direction::NorthEast) {
						if let Cell::ElectronHead = c {
							count += 1;
						}
					}
				}
				if x > 0 {
					if let CellType::WireWorld(c) = g.get_neighbor(x, y, Direction::NorthWest) {
						if let Cell::ElectronHead = c {
							count += 1;
						}
					}
				}
			}
			if y < grid.y_count() -1 {
				if let CellType::WireWorld(c) = g.get_neighbor(x, y, Direction::South) {
					if let Cell::ElectronHead = c {
						count += 1;
					}
				}
				if x < grid.x_count() -1 {
					if let CellType::WireWorld(c) = g.get_neighbor(x, y, Direction::SouthEast) {
						if let Cell::ElectronHead = c {
							count += 1;
						}
					}
				}
				if x > 0 {
					if let CellType::WireWorld(c) = g.get_neighbor(x, y, Direction::SouthWest) {
						if let Cell::ElectronHead = c {
							count += 1;
						}
					}
				}
			}
			if x < grid.x_count() -1 {
				if let CellType::WireWorld(c) = g.get_neighbor(x, y, Direction::East) {
					if let Cell::ElectronHead = c {
						count += 1;
					}
				}
			}
			if x > 0 {
				if let CellType::WireWorld(c) = g.get_neighbor(x, y, Direction::West) {
					if let Cell::ElectronHead = c {
						count += 1;
					}
				}
			}

			if count != 0 && count < 3 {
				grid.set_cell(CellType::WireWorld(Cell::ElectronHead), x, y);
			}
		},
		Cell::Empty => (),
	}
}

pub fn get_color_wireworld(cell: Cell, x: usize, y: usize) -> graphics::Color {
	match cell {
		Cell::ElectronHead => graphics::Color::new(0.0, 0.0, 1.0, 1.0),
		Cell::ElectronTail => graphics::Color::new(1.0, 0.0, 0.0, 1.0),
		Cell::Conductor => graphics::Color::new(1.0, 1.0, 0.0, 1.0),
		_ => graphics::Color::new(0.2, 0.2, 0.2, 1.0),
	}
}