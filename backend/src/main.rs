use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod db;
mod lib;
mod routes;

fn main() {
    // start a very simple HTTP server
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = lib::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512 * 10];
    stream.read(&mut buffer).unwrap();

    // convert incoming HTTP requests into a string
    let raw_request = String::from_utf8_lossy(&buffer[..]);

    // extract relevant information
    let (method, url, payload) = parse_headers_and_body(&raw_request.trim_matches(char::from(0)));

    println!("{}, {}, {}", method, url, payload);

    // route to simple DB handling logic
    let contents = routes::routes(&method, &url, &payload.trim()).unwrap();

    let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_headers_and_body(raw_request: &str) -> (&str, &str, &str) {
    let request: Vec<&str> = raw_request.split("\r\n").collect();

    let first_line: Vec<&str> = request[0].split(" ").collect();
    let method = first_line[0];
    let url = first_line[1];

    let last_line = request[request.len() - 1];
    let payload = last_line;

    (method, url, payload)
}
