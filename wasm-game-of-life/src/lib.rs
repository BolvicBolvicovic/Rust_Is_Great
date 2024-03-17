mod utils;

extern crate fixedbitset;
extern crate js_sys;

use fixedbitset::FixedBitSet;
use std::fmt;
use wasm_bindgen::prelude::*;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
	width: u32,
	height: u32,
	cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
	pub fn tick(&mut self) {
		let mut next = self.cells.clone();

		for row in 0..self.height {
			for col in 0..self.width {
				let idx = self.get_cell_index(row, col);
				let cell = self.cells[idx];
				let alive_neighbors = self.alive_neighbors_count(row, col);
				next.set(idx, match (cell, alive_neighbors) {
					(true, x) if x < 2 => false,
					(true, 2) | (true, 3) => true,
					(true, x) if x > 3 => false,
					(false, 3) => true,
					(otherwise, _) => otherwise,
				});
			}
		}
		self.cells = next;
	}
	
	fn get_cell_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}
	
	fn alive_neighbors_count(&self, row: u32, column: u32) -> u8 {
		let mut count = 0;
		for delta_row in [self.height - 1, 0, 1].iter().cloned() {
			for delta_col in [self.width - 1, 0, 1].iter().cloned() {
				if delta_row == 0 && delta_col == 0 {
					continue ;
				}
			let neighbor_row = (row +delta_row) % self.height;
			let neighbor_col = (column + delta_col) % self.width;
			let idx = self.get_cell_index(neighbor_row, neighbor_col);
			count += self.cells[idx] as u8;
			}
		}
		count
	}

	pub fn new() -> Universe {
		let width = 128;
		let height = 128;

		let size = (width * height) as usize;
		let mut cells = FixedBitSet::with_capacity(size);

		for i in 0..size {
			cells.set(i, js_sys::Math::random() < 0.5);
		}

		Universe {
			width,
			height,
			cells,
		}
	}

	pub fn get_width(&self) -> u32			{ self.width }
	pub fn get_height(&self) -> u32			{ self.height }
	pub fn get_cells(&self) -> *const u32	{ self.cells.as_slice().as_ptr() }
	
	pub fn render (&self) -> String {
		self.to_string()
	}

	pub fn toggle_cell (&mut self, row: u32, column: u32) {
		let idx = self.get_cell_index(row, column);
		self.cells.toggle(idx);
	}
}

impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for line in self.cells.as_slice().chunks(self.width as usize) {
			for &cell in line {
				let symbol = if (cell != 0) == false {'◻'} else {'◼'};
				write!(f, "{}", symbol)?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}
