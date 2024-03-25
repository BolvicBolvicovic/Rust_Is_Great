#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;

use rocket::{
	response::{ content::RawHtml },
	serde::{ Serialize, Deserialize, json::Json },
	form::Form,
};
//use rocket_contrib::databases::diesel;
use std::ops::DerefMut;

//#[database("sqlite_logs")]
//struct LogsDbConn(diesel::SqliteConnection);

#[derive(FromForm, Serialize, Deserialize)]
struct Url {
	#[field(validate=len(1..))]
	url			: String,
	shorten_url	: String,
}

impl Clone for Url {
	fn clone(&self) -> Url {
		Url {
			url			: String::from(self.url.as_str()),
			shorten_url	: String::from(self.url.as_str()),
		}
	}
}

#[get("/")]
fn index() -> RawHtml<&'static str>{
	RawHtml(include_str!("../index.html"))
}

fn to_shorter_url(node: &mut Url) -> &mut Url {
	let end = node.url.rfind(".com").expect("Panic: Input is not a '.com' URL.") + 4;

	node.shorten_url = String::from(&node.url[0..=end]);
	node
}

#[post("/", data = "<form>")]
fn submit(mut form: Form<Url>) -> Json<Url> {
	let url = to_shorter_url(form.deref_mut());
	println!("{}", url.shorten_url);
	Json((*url).clone())
}

#[launch]
fn rocket() -> _ {
	rocket::build()
//		.attach(LogsDbConn::fairing())
		.mount("/", routes![index, submit])
}
