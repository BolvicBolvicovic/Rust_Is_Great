#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod url;

use url::{ Url, GetUrl, UrlId };
use rocket::{
	outcome::Outcome,
	response::{ content::RawHtml },
	form::Form,
};
use rocket_contrib::databases::rusqlite::Connection;
use std::ops::DerefMut;

#[database("db")]
struct Database(Connection);

fn insert_url(conn: &Connection, url: Url) -> usize {
}

fn to_shorten_url(node: &mut GetUrl) -> Url {
	let end	= node.url.rfind(".com").expect("Panic: Input is not a '.com' URL.") + 3;
	Url::new(node.url.clone(), String::from(&node.url[0..=end]))
}

#[get("/")]
fn index(conn: Database) -> RawHtml<&'static str> {
	RawHtml(include_str!("../index.html"))
}

#[post("/", data = "<form>")]
fn submit(mut form: Form<GetUrl>) -> RawHtml<&'static str> {
	let url		= to_shorten_url(form.deref_mut());
	insert_url(&Database, url);
	RawHtml(include_str!("../index.html"))
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index, submit])
		.attach(Database::fairing())
}
