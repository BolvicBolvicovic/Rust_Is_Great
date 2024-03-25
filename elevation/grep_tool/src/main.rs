use std::{
	env,
	fs::File,
	io::{self, BufRead}
};

struct	FullFile{
	file:	File,
	name:	String,
}

fn get_files(mut input: env::Args) -> Vec<FullFile> {
	let mut files = Vec::<FullFile>::new();
	for path in input.by_ref() {
		let file = File::open(path.clone());
		files.push(FullFile {
			file: file.expect("Panic: Path file incorrect."),
			name: path,
		});
	}
	files
}

fn read_pattern(mut input: env::Args) -> (String, env::Args) {
	input.next(); //skipping the executable
	(input.next().expect("Panic: No argument provided."), input)
}

fn find_pattern(pattern: String, files: Vec<FullFile>) {
	for file in files.iter() {
		let mut lines = io::BufReader::new(file.file.try_clone().unwrap()).lines();
		for line in lines.by_ref().enumerate() {
			if line.1.as_ref().expect("Panic: File cannot be read.").contains(&pattern) {
				println!("{} at line {}: {}", file.name, line.0 + 1, line.1.unwrap());
			}
		}
	}
}

fn main() {
	let input = std::env::args();
	let (pattern, input) = read_pattern(input);
	let files = get_files(input);
	find_pattern(pattern, files);
}
