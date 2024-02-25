use std::io::{self, Write};

pub fn get_line() -> String {
	print!("🦀 "); io::stdout().flush().unwrap();
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("...some crab language...");
	line
}
