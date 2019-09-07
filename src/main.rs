mod grid;

use grid::*;

use ggez::{Context, ContextBuilder, GameResult, graphics, mint};
use ggez::event::{self, EventHandler};
use ggez::input::mouse::MouseButton;
use ggez::input::keyboard::{KeyCode, KeyMods};

use ggez::timer;

const CELL_SIZE: usize = 50;

struct State {
	update_cap: u64,
	dt: std::time::Duration,
	pause: bool,
	grid: Grid,
}

impl EventHandler for State {
	fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
		let pos: (usize, usize) = (x as usize / CELL_SIZE, y as usize / CELL_SIZE);
		match button {
			MouseButton::Left => {
				if let CellType::WireWorld(cell) = self.grid.get_cell(pos.0, pos.1) {
					match cell {
						wireworld::Cell::Empty => self.grid.set_cell(
							CellType::WireWorld(wireworld::Cell::Conductor),
							pos.0,
							pos.1,
						),
						wireworld::Cell::Conductor => self.grid.set_cell(
							CellType::WireWorld(wireworld::Cell::ElectronHead),
							pos.0,
							pos.1,
						),
						_ => (),
					}
				}
			},
			MouseButton::Right => self.grid.set_cell(
				CellType::WireWorld(wireworld::Cell::Empty),
				pos.0,
				pos.1,
			),
			_ => (),
		}
	}

	fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
		if keycode == KeyCode::P {
			self.pause = !self.pause;
		}
	}

	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.dt = timer::delta(ctx);

		// Logic
		if !self.pause {
			self.grid.update();
		}

		std::thread::sleep(std::time::Duration::from_nanos(1e9 as u64 / self.update_cap));
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		println!("FPS: {}", 1e9 as u32 / self.dt.subsec_nanos());

		// Render

		for y in 0..self.grid.y_count() {
			for x in 0..self.grid.x_count() {

				// Draw Cells
				let rect = graphics::Mesh::new_rectangle(
					ctx,
					graphics::DrawMode::fill(),
					graphics::Rect::new(
						(CELL_SIZE * x) as f32,
						(CELL_SIZE * y) as f32,
						CELL_SIZE as f32,
						CELL_SIZE as f32
					),
					self.grid.get_color(x, y),
				)?;
				graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

				// Draw Verticle Lines
				let line_vert = graphics::Mesh::new_line(
					ctx,
					&[
						mint::Point2{
							x: (x * CELL_SIZE) as f32,
							y: 0.0,
						},
						mint::Point2{
							x: (x * CELL_SIZE) as f32,
							y: (self.grid.y_count() * CELL_SIZE) as f32,
						},
					],
					1.0,
					graphics::WHITE,
				)?;
				graphics::draw(ctx, &line_vert, graphics::DrawParam::default())?;
			}

			// Draw Horizontal Lines
			let line_hori = graphics::Mesh::new_line(
				ctx,
				&[
					mint::Point2{
						x: 0.0,
						y: (y * CELL_SIZE) as f32,
					},
					mint::Point2{
						x: (self.grid.x_count() * CELL_SIZE) as f32,
						y: (y * CELL_SIZE) as f32,
					},
				],
				1.0,
				graphics::WHITE,
			)?;
			graphics::draw(ctx, &line_hori, graphics::DrawParam::default())?;
		}

		graphics::present(ctx)?;
		Ok(())
	}
}

pub fn main() {
	use ggez::conf::*;

	// Initialize
	let grid = Grid::new(20, 20, CellType::WireWorld(wireworld::Cell::Empty));

	let state = &mut State {
		update_cap: 15,
		dt: std::time::Duration::new(0, 0),
		pause: false,
		grid,
	};

	let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Cellular Automata", "Michael McCarthy")
		.window_setup(
			WindowSetup::default()
				.title("Cellular Automata")
				.samples(NumSamples::Zero)
				.vsync(true),
		)
		.window_mode(WindowMode::default().dimensions(
			(state.grid.x_count() * CELL_SIZE) as f32,
			(state.grid.y_count() * CELL_SIZE) as f32
		))
		.build()
		.unwrap();

	event::run(ctx, event_loop, state).unwrap();
}
