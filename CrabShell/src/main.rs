#![allow(non_snake_case)]

mod utils;
use utils::*;

fn main() /*-> Result<()>*/ {
	loop {
		let line = get_line();
		if line.starts_with('\n') { break; }

		println!("The line you just wrote: {}", line);
	}	

	//Ok(())
}
