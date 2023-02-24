use crate::md_parser::parse_md;

pub mod md_parser;

use std::{
    error, fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

// override result type
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct HTTPResponse {
    status: String,
    headers: String,
    body: Vec<u8>,
}

trait FormatResponse {
    // return byte array - could be text but might want to extend to images in future
    fn into_response(&mut self) -> Vec<u8>;
}

impl FormatResponse for HTTPResponse {
    fn into_response(&mut self) -> Vec<u8> {
        match self {
            HTTPResponse {
                status,
                headers,
                body,
            } => {
                let mut response = format!("{status}\r\n{headers}\r\n\r\n").as_bytes().to_vec();
                response.append(body);
                return response;
            }
        }
    }
}

// The content directory to server -should end in '/'
const SERVER_DIR: &str = "/home/jack/rust/webserver/src/pub/";
const STATUS_OK: &str = "HTTP/1.1 200 OK ";
const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 Not Found ";
const GLOBAL_STYLE: &str = "
	h1{
		font-size: 2.5em;
	}
	body{
		font-family:helvetica;
	}
	.container{
		width: 50%;
		margin: 40px auto;
	}
	.code{
		white-space: pre;
		tab-size: 30px;
		font-family:monospace;
	}";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let conn_result = handle_connection(stream);
        if conn_result.is_err() {
            println!("Connection error.");
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);

    // A response to send if the resource isn't found
    let mut error_response = HTTPResponse {
        status: STATUS_NOT_FOUND.to_string(),
        headers: "".to_string(),
        body: "404 // Requested resource not found"
            .to_string()
            .into_bytes(),
    };

    // Split the request into parts and get the requested resource
    let mut request = String::new();
    buf_reader.read_line(&mut request)?;

    let parts: Vec<_> = request.split(" ").collect();
    let resource = parts[1];

    let resolve_result = resolve_response(resource);

    let response = match resolve_result {
        Ok(response) => response,
        Err(_) => error_response.into_response(),
    };

    stream.write_all(&response)?;
    Ok(())
}

/**
 * Takes a requested resource and tries to resolve it to something on the server
 *
 */
fn resolve_response(requested_resource: &str) -> Result<Vec<u8>> {
    let mut path = PathBuf::from(requested_resource);
    let has_extension = path.extension().is_some();

    // TODO - If a file extension has been provided, load that resource if available
    if has_extension {
        let mut file_response = load_file_response(path)?;
        return Ok(file_response.into_response());
    }

    // Automatically resolve the root to index
    if path.as_os_str() == "/" {
        path = PathBuf::from("/index");
    }

    path.set_extension("md");

    let mut md_response = load_md_response(path)?;
    Ok(md_response.into_response())
}

/**
 * Loads and parses a markdown file relative to the server directory
 */
fn load_md_response(path: PathBuf) -> Result<HTTPResponse> {
    let mut abs_path = PathBuf::from(SERVER_DIR);
    abs_path.push(path.strip_prefix("/")?.to_path_buf());

    println!("Requested {:#?}...", abs_path);

    let read_result = fs::read_to_string(abs_path)?;
    let md_parse_result = parse_md(read_result);
    let html = format!(
        "
	<head>
		<title>{title}</title>
		<style>
			{GLOBAL_STYLE}
		</style>
	</head>
	<body>
		<div class=\"container\">
			{md_parse_result}
		</div>
	</body>
	",
        title = path.strip_prefix("/")?.display()
    );
    let length = html.len().to_string();

    let response = HTTPResponse {
        status: STATUS_OK.to_string(),
        headers: format!("Content-Length: {length}"),
        body: html.into_bytes(),
    };

    return Ok(response);
}

fn load_file_response(path: PathBuf) -> Result<HTTPResponse> {
    let extension = path.extension();

    // Match different extensions for content type here
    let header = match extension {
        Some(os_str) => match os_str.to_str() {
            Some("ico") => "Content-Type: image/*",
            Some("png") => "Content-Type: image/png",
            Some(&_) => "Content-Type: text/html",
            None => "",
        },
        None => "",
    };

    let mut abs_path = PathBuf::from(SERVER_DIR);
    abs_path.push(path.strip_prefix("/")?.to_path_buf());

    println!("Requested {:#?}...", abs_path);

    let read_result = fs::read(abs_path)?;
    let length = read_result.len().to_string();

    let response = HTTPResponse {
        status: STATUS_OK.to_string(),
        headers: format!("{header}\nContent-Length: {length}"),
        body: read_result,
    };

    return Ok(response);
}
