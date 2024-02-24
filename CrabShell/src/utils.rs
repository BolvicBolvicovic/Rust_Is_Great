use std::io;

pub fn get_line() -> String {
	println!("🦀 ");
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("...some crab language...");
	line
}
