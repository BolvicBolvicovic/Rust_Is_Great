extern crate fixedbitset;
extern crate js_sys;

use lazy_static::lazy_static;
use fixedbitset::FixedBitSet;
use std::fmt;
use wasm_bindgen::prelude::*;
use std::mem;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static!{
static ref	ENTITIES: Vec<Vec<u32>> = vec![vec![		//Glider
						0,0,0,0,0,
						0,0,1,0,0,
						0,0,0,1,0,
						0,1,1,1,0,
						0,0,0,0,0,
					],
					vec![								//Pentadecathlon
						0,0,1,0,0,0,0,1,0,0,
						1,1,0,1,1,1,1,0,1,1,
						0,0,1,0,0,0,0,1,0,0,
					],
					vec![								//Pulsar
						0,0,1,1,1,0,0,0,1,1,1,0,0,
						0,0,0,0,0,0,0,0,0,0,0,0,0,
						1,0,0,0,0,1,0,1,0,0,0,0,1,
						1,0,0,0,0,1,0,1,0,0,0,0,1,
						1,0,0,0,0,1,0,1,0,0,0,0,1,
						0,0,1,1,1,0,0,0,1,1,1,0,0,
						0,0,0,0,0,0,0,0,0,0,0,0,0,
						0,0,1,1,1,0,0,0,1,1,1,0,0,
						1,0,0,0,0,1,0,1,0,0,0,0,1,
						1,0,0,0,0,1,0,1,0,0,0,0,1,
						1,0,0,0,0,1,0,1,0,0,0,0,1,
						0,0,0,0,0,0,0,0,0,0,0,0,0,
						0,0,1,1,1,0,0,0,1,1,1,0,0,
					]];
}

#[wasm_bindgen]
pub struct Universe {
	width:	u32,
	height:	u32,
	cells:	FixedBitSet,
	next:	FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
	pub fn tick(&mut self) {
		for row in 0..self.height {
			for col in 0..self.width {
				let idx = self.get_cell_index(row, col);
				let cell = self.cells[idx];
				let alive_neighbors = self.alive_neighbors_count(row, col);
				self.next.set(idx, match (cell, alive_neighbors) {
					(true, x) if x < 2 => false,
					(true, 2) | (true, 3) => true,
					(true, x) if x > 3 => false,
					(false, 3) => true,
					(otherwise, _) => otherwise,
				});
			}
		}
		mem::swap(&mut self.cells, &mut self.next);
	}
	
	fn get_cell_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}
	
	fn alive_neighbors_count(&self, row: u32, column: u32) -> u8 {
		let mut count = 0;
	// THIS IS FASTER MF
		let north = if row == 0 {
    	    	self.height - 1
    		} else {
    	    	row - 1
    	};

    	let south = if row == self.height - 1 {
				0
    		} else {
    		    row + 1
    	};

    	let west = if column == 0 {
    	    	self.width - 1
    		} else {
    	    	column - 1
    	};

    	let east = if column == self.width - 1 {
    		    0
    		} else {
    		    column + 1
    	};
    	let nw = self.get_cell_index(north, west);
    	count += self.cells[nw] as u8;

    	let n = self.get_cell_index(north, column);
    	count += self.cells[n] as u8;

    	let ne = self.get_cell_index(north, east);
    	count += self.cells[ne] as u8;

    	let w = self.get_cell_index(row, west);
    	count += self.cells[w] as u8;

    	let e = self.get_cell_index(row, east);
    	count += self.cells[e] as u8;

    	let sw = self.get_cell_index(south, west);
    	count += self.cells[sw] as u8;

    	let s = self.get_cell_index(south, column);
    	count += self.cells[s] as u8;

    	let se = self.get_cell_index(south, east);
		count += self.cells[se] as u8;
	
		count
	}

	pub fn new() -> Universe {
		let width	= 64;
		let height	= 64;

		let size		= (width * height) as usize;
		let mut cells	= FixedBitSet::with_capacity(size);
		let mut next	= FixedBitSet::with_capacity(size);

		for i in 0..size {
			cells.set(i, false);
			next.set(i, false);
		}

		Universe {
			width,
			height,
			cells,
			next,
		}
	}

	pub fn get_width(&self) -> u32			{ self.width }
	pub fn get_height(&self) -> u32			{ self.height }
	pub fn get_cells(&self) -> *const u32	{ self.cells.as_slice().as_ptr() }
	
	pub fn render (&self) -> String {
		self.to_string()
	}

	pub fn clear_cells(&mut self) {
		self.cells.clear();
	}

	pub fn toggle_cell(&mut self, row: u32, column: u32) {
		let idx = self.get_cell_index(row, column);
		self.cells.toggle(idx);
	}

	pub fn random_restart(&mut self) {
		self.clear_cells();
		for i in 0..(self.width * self.height) {
			self.cells.set(i as usize, js_sys::Math::random() < 0.5);
		}
	}

	pub fn glider(&mut self, row: u32, column: u32) {
		if row > 3 && column > 3 && row < self.width - 3 && column < self.height - 3 {
			let row_offset: u32 = row - 2;
			let col_offset: u32 = column - 2;
			for i in row_offset..(row_offset + 5) {
				for j in col_offset..(col_offset + 5) {
					let idx = self.get_cell_index(i, j);
					if self.cells[idx] {
						self.cells.toggle(idx);
					}
					/*if ENTITIES[0][(i * 5 + j) as usize] == 1 {
						{ self.cells.toggle(idx); }
					}*/
					if	(i == row_offset + 1 && j == col_offset + 2) ||
						(i == row_offset + 2 && j == col_offset + 3) ||
						(i == row_offset + 3 && (j == col_offset + 1 ||
												j == col_offset + 2 ||
												j == col_offset + 3))
						{ self.cells.toggle(idx); }
				}
			}
		}
	}
	
	pub fn pulsar(&mut self, row: u32, column: u32) {
		if row > 7 && column > 7 && row < self.width - 7 && column < self.height - 7 {
			let row_offset: u32 = row - 6;
			let col_offset: u32 = column - 6;
			for i in row_offset..(row_offset + 13) {
				for j in col_offset..(col_offset + 13) {
					let idx = self.get_cell_index(i, j);
					if self.cells[idx] {
						self.cells.toggle(idx);
					}
					if	((	i == row_offset		||
							i == row_offset + 5 ||
							i == row_offset + 7 ||
							i == row_offset + 12) &&
						(	j == col_offset + 2 ||
							j == col_offset + 3 ||
							j == col_offset + 4 ||
							j == col_offset + 8 ||
							j == col_offset + 9 ||
							j == col_offset + 10)) ||
						((	i == row_offset + 2 ||
							i == row_offset + 3 ||
							i == row_offset + 4 ||
							i == row_offset + 8 ||
							i == row_offset + 9 ||
							i == row_offset + 10) &&
						(	j == col_offset 	||
							j == col_offset + 5 ||
							j == col_offset + 7 ||
							j == col_offset + 12))
						{ self.cells.toggle(idx); }
				}
			}
		}
	}
	
	pub fn pentadecathlon(&mut self, row: u32, column: u32) {
		if row > 12 && column > 12 && row < self.width - 12 && column < self.height - 12 {
			let row_offset: u32 = row - 1;
			let col_offset: u32 = column - 4;
			for i in row_offset..(row_offset + 3) {
				for j in col_offset..(col_offset + 10) {
					let idx = self.get_cell_index(i, j);
					if self.cells[idx] {
						self.cells.toggle(idx);
					}
					if	((	i == row_offset 	||
							i == row_offset + 2) &&
						(	j == col_offset + 2 ||
							j == col_offset + 7)) ||
						(	i == row_offset + 1 &&
						(	j == col_offset 	||
							j == col_offset + 1 ||
							j == col_offset + 3 ||
							j == col_offset + 4 ||
							j == col_offset + 5 ||
							j == col_offset + 6 ||
							j == col_offset + 8 ||
							j == col_offset + 9))
						{ self.cells.toggle(idx); }
				}
			}
		}
	}
}

impl Default for Universe {
	fn default() -> Self {
		Self::new()
	}
}

impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for line in self.cells.as_slice().chunks(self.width as usize) {
			for &cell in line {
				let symbol = if cell == 0 {'◻'} else {'◼'};
				write!(f, "{}", symbol)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}
