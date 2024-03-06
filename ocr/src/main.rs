use std::fs;

const DATA_FILENAME_LAB: &str	= "data/t10k-labels.idx1-ubyte";
const DATA_FILENAME_IMG: &str	= "data/t10k-images.idx3-ubyte";
const TRAIN_FILENAME_LAB: &str	= "data/train-labels.idx1-ubyte";
const TRAIN_FILENAME_IMG: &str	= "data/train-images.idx3-ubyte";

pub fn bytes_to_i32(bytes: &[u8]) -> usize {
	((bytes[0] as usize) << 24) |
	((bytes[1] as usize) << 16) |
	((bytes[2] as usize) <<  8) |
	( bytes[3] as usize)
}

pub struct File {
	content	: Vec<u8>,
	name	: String,
	images	: Vec<Vec<Vec<u8>>>
}
impl File {
	fn new(filename: &str) -> File {
		let mut my_imgs = Vec::new();
		let cont = fs::read(filename).expect("Should be able to read file.");
        let mut i = 0;

		if filename.find("image").is_some() {
			let n_img	= bytes_to_i32(&cont[4..=7]);
			let n_r		= bytes_to_i32(&cont[8..=11]);
			let n_c		= bytes_to_i32(&cont[12..=15]);
			
			for _img_idx in 0..n_img {
				let mut image = Vec::new();
				for _row_idx in 0..n_r {
					let mut row = Vec::new();
					for _col_idx in 0..n_c {
						row.push(cont[i + 16]);
                        i += 1;
					}
					image.push(row);
				}
				my_imgs.push(image);
			}
		} else {
			my_imgs	= Vec::new();
		}
		File {
			content : Vec::from(cont),
			name	: String::from(filename),
			images	: my_imgs,
		}
	}

	fn print_struct(&self) {
		println!("{} {}\n", self.name, self.images.len());
	}
}

fn main() {
	let x_test  = File::new(&DATA_FILENAME_IMG);
	let y_test  = File::new(&DATA_FILENAME_LAB);
    let x_train = File::new(&DATA_FILENAME_IMG);
    let y_train = File::new(&DATA_FILENAME_LAB);
}
