use std::fs;

const DATA_FILENAME_LAB: &str	= "data/t10k-labels.idx1-ubyte";
const DATA_FILENAME_IMG: &str	= "data/t10k-images.idx3-ubyte";
const TRAIN_FILENAME_LAB: &str	= "data/train-labels.idx1-ubyte";
const TRAIN_FILENAME_IMG: &str	= "data/train-images.idx3-ubyte";

fn bytes_to_i32(bytes: &[u8]) -> i32 {
	((bytes[0] as i32) << 24) |
	((bytes[1] as i32) << 16) |
	((bytes[2] as i32) <<  8) |
	( bytes[3] as i32)
}

fn read_data(filename: &str) {
	let content			= fs::read(filename).expect("Should be able to read file.");
	let n_images:	i32	= bytes_to_i32(&content[4..=7]);
	let n_row:		i32	= bytes_to_i32(&content[8..=11]);
	let n_col:		i32	= bytes_to_i32(&content[12..=15]);
	println!("{:?}\n{:?}\n{:?}", n_images, n_row, n_col);
}

fn main() {
	read_data(&DATA_FILENAME_IMG);
}
