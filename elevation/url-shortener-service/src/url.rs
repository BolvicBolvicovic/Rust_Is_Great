mod urls;

use rocket::serde::Serialize;
use std::{
	path::{ Path, PathBuf },
};
use rand::Rng;

#[derive(FromForm)]
pub struct GetUrl {
	#[field(validate=len(1..))]
	pub url: String,
}

impl Clone for GetUrl {
	fn clone(&self) -> GetUrl {
		GetUrl{ url: String::from(self.url.as_str()) }	
	}
}

#[derive(Serialize, diesel::Queryable, diesel::Insertable)]
pub struct Url {
	pub url_id		: UrlId,
	pub url			: String,
	pub shorten_url	: String,
}

impl Url {
	pub fn new(r: String, s: String) -> Url {
		Url {
			url_id		: UrlId::new(10),
			url			: r,
			shorten_url	: s,
		}
	}
}

impl Clone for Url {
	fn clone(&self) -> Url {
		Url {
			url_id		: self.url_id.clone(),
			url			: String::from(self.url.as_str()),
			shorten_url	: String::from(self.shorten_url.as_str()),
		}
	}
}

#[derive(Serialize)]
pub struct UrlId{ id: String }

impl UrlId {
	pub fn new(size: usize) -> UrlId {
		const BASE62: &[u8]	= b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
		let mut id			= String::with_capacity(size);
		let mut rng			= rand::thread_rng();

		for _ in 0..size {
			id.push(BASE62[rng.gen::<usize>() % 62] as char);
		}
		UrlId{ id : id, }
	}
//TODO: change file_path so I actualy redirects to the database instead of a dir that contains lots of files
	pub fn file_path(&self) -> PathBuf {
		let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
		Path::new(root).join(String::from(self.id.as_str()))
	}

	pub fn get_id(&self) -> String {
		String::from(self.id.as_str())
	}
}

impl Clone for UrlId {
	fn clone(&self) -> UrlId {
		UrlId{ id: String::from(self.id.as_str()), }
	}
}
