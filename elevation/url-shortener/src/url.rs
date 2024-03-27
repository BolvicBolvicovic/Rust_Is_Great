use rocket::serde::Serialize;

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

#[derive(Serialize)]
pub struct Url {
	pub url			: String,
	pub shorten_url	: String,
}

impl Url {
	pub fn new(r: String, s: String) -> Url {
		Url {
			url			: r,
			shorten_url	: s,
		}
	}
}

impl Clone for Url {
	fn clone(&self) -> Url {
		Url {
			url			: String::from(self.url.as_str()),
			shorten_url	: String::from(self.shorten_url.as_str()),
		}
	}
}
