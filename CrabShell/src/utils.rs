use std::io::{self, Write};

pub fn is_wspace_but_nl(c: char) -> bool {
	match c {
		'\n' => false,
		_ => c.is_ascii_whitespace(),
	}
}

pub fn get_line() -> String {
	print!("🦀 "); io::stdout().flush().unwrap();
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("...some crab language...");
	line
}
