#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
mod url;

use rusqlite::{ Connection };
use url::{ Url, GetUrl };
use std::collections::HashMap;
use rocket::{
	response::{ content::RawHtml },
	form::Form,
};
use std::ops::DerefMut;

fn insert_url(conn: &Connection, url: Url) {
	let mut	hm_url = HashMap::new();
	hm_url.insert(String::from(url.url.as_str()), String::from(url.shorten_url.as_str()));
	conn.execute(
		"INSERT INTO urls (url, shorten_url) VALUES (?1, ?2)",
		&[&url.url, &url.shorten_url],
	).expect("Panic: Could not insert data in database");
}

fn to_shorten_url(node: &mut GetUrl) -> Url {
	let end	= node.url.rfind(".com").expect("Panic: Input is not a '.com' URL.") + 3;
	Url::new(node.url.clone(), String::from(&node.url[0..=end]))
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
	RawHtml(include_str!("../index.html"))
}

#[post("/", data = "<form>")]
fn submit(mut form: Form<GetUrl>) -> RawHtml<&'static str> {
	let connection: Connection	= Connection::open("./database/database.sqlite")
		.expect("Panic: Could not open database");
	let url		= to_shorten_url(form.deref_mut());
	insert_url(&connection, url);
	RawHtml(include_str!("../index.html"))
}

#[launch]
fn rocket() -> _ {
	let connection: Connection	= Connection::open("./database/database.sqlite")
		.expect("Panic: Could not open database");
	connection.execute(
		"CREATE TABLE if not exists urls (
			id			INTEGER PRIMERY KEY,
			url			TEXT NOT NULL,
			shorten_url TEXT NOT NULL
		)",
		(),
	).expect("Panic: Cound not create table in database");
	rocket::build()
		.mount("/", routes![index, submit])
}
