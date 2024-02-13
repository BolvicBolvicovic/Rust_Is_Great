struct Cli {
	pattern: String,
	path: std::path::PathBuf,
}

fn main() {
	let pattern = std::env::args().nth(1).expect("No pattern given");
	let path = std::env::args().nth(1).expect("No path given");

    println!("pattern: {:?}, path: {:?}", pattern, path)
}
