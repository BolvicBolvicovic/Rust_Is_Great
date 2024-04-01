# URL Shortener

This was a journey with rocket, tera and rusqlite.
The objective was to make a server run with rocket, receive HTTP request from the browser,
interact with the database and give a response.

## Installation

Once you are in the project folder you just need to run the following command:
```bash
cargo run
```

## Usage

It is quite straight forward as it is designed as if it was made by a back-end dev.
You type the url, click on the button *shorten!*
and you get a shorten URL that you can click on.
You also have a delete button that will erase the shortened URL
and the URL itself from the database.
