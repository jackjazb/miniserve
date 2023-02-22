use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
		handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream){
	let buf_reader = BufReader::new(&mut stream);
	let http_request: Vec<_> = buf_reader
		.lines()
		.map(|result| result.unwrap())
		.take_while(|line| !line.is_empty())
		.collect();

	let response = "
		HTTP/1.1 200 OK 
		Content-Type: text/html\n
		<!DOCTYPE html>
		<html>
			<h1>testing</h1>
		</html>
		\r\n\r\n";
	stream.write_all(response.as_bytes()).unwrap();
}