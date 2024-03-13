use std::fs;

const DATA_FILENAME_LAB: &str	= "data/t10k-labels.idx1-ubyte";
const DATA_FILENAME_IMG: &str	= "data/t10k-images.idx3-ubyte";
const TRAIN_FILENAME_LAB: &str	= "data/train-labels.idx1-ubyte";
const TRAIN_FILENAME_IMG: &str	= "data/train-images.idx3-ubyte";

pub fn		bytes_to_i32(bytes: &[u8]) -> usize {
	((bytes[0] as usize) << 24) |
	((bytes[1] as usize) << 16) |
	((bytes[2] as usize) <<  8) |
	( bytes[3] as usize)
}

#[derive(Debug)]
pub enum	FileType {
	Image,
	Label,
	Error,
}

pub struct	File {
	content	: Vec<u8>,
	name	: String,
	_type	: FileType,
	images	: Vec<Vec<Vec<u8>>>,
}
impl 	File {
	fn	new(filename: &str) -> File {
		if filename.rfind("-ubyte") == None {
			return File {
				content	: Vec::new(),
				name	: String::from(filename),
				_type	: FileType::Error,
				images	: Vec::new(),
			};
		}
		let mut my_imgs = Vec::new();
		let cont = fs::read(filename).expect("Should be able to read file.");
        let mut i = 0;
		let mut _type = FileType::Label;

		if filename.find("image").is_some() {
			let n_img	= bytes_to_i32(&cont[4..=7]);
			let n_r		= bytes_to_i32(&cont[8..=11]);
			let n_c		= bytes_to_i32(&cont[12..=15]);
			_type = FileType::Image;
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
			let v1		= Vec::from(&cont[8..]);
 			let mut v2	= Vec::new();
			v2.push(v1);
			my_imgs.push(v2);
		}
		File {
			content : Vec::from(cont),
			name	: String::from(filename),
			_type	: _type,
			images	: my_imgs,
		}
	}

	fn	features(&self) -> Result<Vec<Vec<u8>>, FileType> {
		match self._type {
			FileType::Image	=> {
				let n_img	= bytes_to_i32(&self.content[4..=7]);
				let pixels	= bytes_to_i32(&self.content[8..=11]) * bytes_to_i32(&self.content[12..=15]);
				let mut i	= 0;
				let mut result	= Vec::new();
				for _img in 0..n_img {
					let mut sample = Vec::new();
					for _pixel in 0..pixels {
						sample.push(self.content[i + 16]);
						i += 1;
					}
					result.push(sample);
				}
				Ok(result)
			},
			_				=> Err(FileType::Error)
		}
	}

	fn	getLabels(&self) -> Result<Vec<u8>, FileType> {
		match self._type {
			FileType::Label => Ok(self.images[0][0].clone()),
			_				=> Err(FileType::Error)
		}
	}
}

pub fn zip(x: Vec<u8>, y: &Vec<u8>) -> Result<Vec<(u8, u8)>, ()> {
	if x.len() != (*y).len() { return Err(())}
	let mut zipped = Vec::new();
	for i in 0..x.len() {
		zipped.push((x[i], (*y)[i]));
	}
	Ok(zipped)
}

pub fn dist(x: Vec<u8>, y: &Vec<u8>) -> u16 {
	let mut distance: u16	= 0;
	let zipped				= zip(x, y).expect("Lists do not have the same length.");
	for (x_i, y_i) in zipped.iter() {
		let calculus	= (x_i - y_i).pow(2) as u16;
		distance		= distance + calculus;
	}
	let mut res: f32	= distance as f32;
	res					= res.sqrt();
	res as u16
}


pub fn get_training_distances_for_test_sample(x_train: &Vec<Vec<u8>>,
											  test_sample: Vec<u8>)
											  -> Vec<u8> {
	let mut distances = Vec::new();
	for train_sample in x_train.iter() {
		distances.push(dist(train_sample.to_vec(), &test_sample).try_into().unwrap());
	}
	distances
}

pub fn enumerate(simple_vector: Vec<u8>) -> Vec<(u8, u8)> {
	let mut vec_enumerated = Vec::new();
	for i in 0..simple_vector.len() {
		vec_enumerated.push((simple_vector[i], i as u8));
	}
	vec_enumerated
}

pub fn sort_and_return_indices(mut indices: Vec<(u8, u8)>) -> Vec<u8> {
	indices.sort_by(|a, b| a.0.cmp(&b.0));
	let mut res	= Vec::new();
	for (_x, y) in indices.iter() {
		res.push(*y)
	}
	res
}

pub fn	get_candidates(sorted_dist: Vec<u8>, y_train: Vec<u8>, k: usize) -> Vec<u8> {
	let mut res = Vec::new();
	for idx in Vec::from(sorted_dist)[0..=k].iter() {
		res.push(y_train[*idx as usize]);
	}
	res
}

pub fn	k_nearest_neighbours(x_train: Vec<Vec<u8>>	,
							 y_train: Vec<u8>		,
							 x_test : Vec<Vec<u8>>	,
							 k		: u8) -> Vec<u8> {
	let mut y_prediction = Vec::new();
	for sample in x_train.iter() {
		let training_distances		= get_training_distances_for_test_sample(&x_train, sample.to_vec());
		let sorted_distance_indices	= sort_and_return_indices(enumerate(training_distances));
		let candidates				= get_candidates(sorted_distance_indices, y_train.clone(), k.into());
		println!("{:?}\n", candidates);
		y_prediction.push(k);
	}
	y_prediction
}

fn	main() {
	let x_test_file		= File::new(&DATA_FILENAME_IMG);
	//let y_test_file		= File::new(&DATA_FILENAME_LAB);
    let x_train_file	= File::new(&DATA_FILENAME_IMG);
    let y_train_file	= File::new(&DATA_FILENAME_LAB);
	let x_test_feat		= x_test_file.features().expect("wrong file type");
	let x_train_feat	= x_train_file.features().expect("wrong file type");
	let y_train_label	= y_train_file.getLabels().expect("wrong file type");
	//let y_test_label	= y_test_file.getLabels().expect("wrong file type");
	k_nearest_neighbours(x_train_feat, y_train_label, x_test_feat, 3);
}
