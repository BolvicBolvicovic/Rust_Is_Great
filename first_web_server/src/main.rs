use std::{
	fs,
	io::{prelude::*, BufReader},
	net::{TcpListener, TcpStream},
	thread,
	time::Duration,
};
use first_web_server::ThreadPool;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").expect("Binding failed!");
	let pool = ThreadPool::new(10);
	for stream in listener.incoming() {
		let stream = stream.expect("Stream error!");
		pool.execute(|| {
		handle_connection(stream);
		});
	}
}

fn handle_connection(mut stream: TcpStream) {
	let buf_reader = BufReader::new(&mut stream);
	let request_line = buf_reader.lines().next().unwrap().unwrap();

	let (status_line, filename) = match &request_line[..] {
	"GET / HTTPS/1.0" => ("HTTP/1.0 200 OK", "index.html"),
	"GET /sleep HTTPS/1.0" => {
		thread::sleep(Duration::from_secs(5));
		("HTTPS/1.0 200 OK", "index.html")}
	_ => ("HTTPS/1.0 404 ERROR NOT FOUND", "error.html")
	};	
	let contents = fs::read_to_string(filename).expect("Failed to read to string!");
	let length = contents.len();
	let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
	stream.write_all(response.as_bytes()).expect("Unsuccessful response!");
}
