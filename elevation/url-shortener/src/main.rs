#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
mod url;

use rocket_dyn_templates::{ context, Template };
use rocket_sync_db_pools::{ database, rusqlite::Connection };
use url::{ Url, GetUrl };
use std::{ 
    ops::DerefMut,
    collections::HashMap,
    path::PathBuf,
};
use rocket::form::Form;

#[database("sqlite_db")]
struct	DatabaseConnection(Connection);

fn insert_url(conn: &Connection, url: Url) {
	let mut	hm_url = HashMap::new();
	hm_url.insert(String::from(url.url.as_str()), String::from(url.shorten_url.as_str()));
	match conn.execute(
		"INSERT INTO urls (url, shorten_url) VALUES (?1, ?2)",
		[&url.url, &url.shorten_url]) {
        Err(e) => println!("Panic: Could not insert data in database: {:?}", e),
        _   => println!("Data saved in database")
    };
}

fn to_shorten_url(node: &mut GetUrl) -> Url {
    let start = match node.url.find("://") {
        Some(start) => start + 3,
        None => { 
            let mut new_url = "https://".to_string();
            new_url.push_str(node.url.as_str()); 
            node.url = new_url;
            8
            },
    };
	let end	= match node.url.rfind(".com") {
        Some(end)   => end + 3,
        None        => match node.url.rfind(".fr") {
            Some(end)   => end + 2,
            None        => {
                node.url.push_str(".com");
                node.url.rfind(".com").unwrap() + 3
            },
        }
    };
	Url::new(node.url.clone(), String::from(&node.url[start..=end]))
}

fn render(conn: &Connection) -> Template {
    let mut stmt = conn.prepare("SELECT url, shorten_url FROM urls")
        .expect("Panic: Coud not prepare connection statement for database");
    let urls = stmt.query_map([], |row| {
        Ok(Url {
            url: row.get(0).unwrap(),
            shorten_url: row.get(1).unwrap(),
        })
    }).unwrap().map(|url| url.unwrap());
    let mut urls_vec = Vec::new();
    for url in urls {
        urls_vec.push(url);
    }
    Template::render("index", context!{ urls: urls_vec })
}

fn delete_data(conn: &Connection, url: String) {
    conn.execute("DELETE FROM urls WHERE shorten_url = (?1)", [&url])
        .expect("Panic: Could not delete data from database");
}

#[get("/")]
async fn index(conn: DatabaseConnection) -> Template {
    conn.run(|c| render(c)).await
}

#[post("/submit", data = "<form>")]
async fn submit(conn: DatabaseConnection, mut form: Form<GetUrl>) -> Template {
	let url		= to_shorten_url(form.deref_mut());
	conn.run(|c| insert_url(c, url)).await;
    conn.run(|c| render(c)).await
}

#[delete("/delete/<name>")]
async fn delete(conn: DatabaseConnection, name: PathBuf) -> Template {
    let string_url = name.display().to_string();
    conn.run(move |c| delete_data(c, string_url)).await;
    conn.run(|c| render(c)).await
}

#[launch]
fn rocket() -> _ {
	let connection: Connection	= Connection::open("./database/database.sqlite")
		.expect("Panic: Could not open database");
	connection.execute_batch(
		"CREATE TABLE if not exists urls (
			id			INTEGER PRIMERY KEY,
			url			TEXT NOT NULL,
			shorten_url TEXT NOT NULL
		)")
		.expect("Panic: Cound not create table in database");
	rocket::build()
        .attach(Template::fairing())
		.attach(DatabaseConnection::fairing())
		.mount("/", routes![index, submit, delete])
}
