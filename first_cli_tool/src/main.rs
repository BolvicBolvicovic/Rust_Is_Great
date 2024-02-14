use clap::Parser;
use std::io::{BufReader, BufRead, {self, Write}};
use std::fs::File;
use anyhow::{Context, Result};
///Parse the command line into the struct through pattern and path.
///If there is not the right amount of args, exits with error msg.
#[derive(Parser)]
struct Cli {
	pattern: String,
	path: std::path::PathBuf,
}

fn main() -> Result<()> {
	let args = Cli::parse();
	
	let file = File::open(&args.path)
		.with_context(|| format!("Could not read file {:?}", args.path.display()))?;
	let content = BufReader::new(file);
	
	let mut count_line = 0;
	let mut handle = io::BufWriter::new(io::stdout());
	
	for line in content.lines() {
		let my_str = line?;
		count_line += 1;
		if my_str.contains(&args.pattern) {
			writeln!(handle, "{2:?}: line {1}: {0}", my_str, &count_line, args.path.display())?;
		}
	handle.flush()?;
	}
	Ok(())
}
