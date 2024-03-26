#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

mod url;
mod urls;

use url::{ Url, GetUrl, UrlId };
use rocket::{
	response::{ content::RawHtml },
	form::Form,
};
use rocket_contrib::databases::diesel::{ prelude::*, SqliteConnection, Connection };
use std::ops::DerefMut;


#[get("/")]
fn index() -> RawHtml<&'static str> {
	RawHtml(include_str!("../index.html"))
}

fn insert_url(conn: &SqliteConnection, url: Url) -> usize {

	diesel::insert_into(urls::urls)
		.values(&url)
		.execute(conn)
		.expect("Panic: Failed to save url in the database")
}

fn to_shorten_url(node: &mut GetUrl) -> Url {
	let end	= node.url.rfind(".com").expect("Panic: Input is not a '.com' URL.") + 3;
	Url::new(node.url.clone(), String::from(&node.url[0..=end]))
}

#[post("/", data = "<form>")]
fn submit(mut form: Form<GetUrl>) -> RawHtml<&'static str> {
	let url		= to_shorten_url(form.deref_mut());
	let conn	= SqliteConnection::establish("/database/database.sqlite").expect("Panic: Could not connect to database");
	insert_url(&conn, url);
	RawHtml(include_str!("../index.html"))
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.mount("/", routes![index, submit])
}
