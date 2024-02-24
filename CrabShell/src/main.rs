mod utils;
use utils::*;

#[allow(non_snake_case)]
fn main() /*-> Result<()>*/ {
	loop {
		let line = get_line();
		if line.is_empty() { break; }
		println!("The line you just wrote: {}", line);
	}	

	//Ok(())
}
