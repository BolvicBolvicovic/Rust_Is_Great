diesel::table! {
	urls (id) {
		id			-> Varchar,
		url			-> Varchar,
		shorten_url	-> Varchar,
	}
}
