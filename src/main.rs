use crate::md_parser::parse_md;

pub mod md_parser;

use std::{
    error, fmt, fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

// override result type
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct ResponseError(String);

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP response error: {}", self.0)
    }
}

struct TextResponse {
    status: String,
    headers: String,
    body: String,
}

trait FormatResponse {
    // return byte array - could be text but might want to extend to images in future
    fn format_response(&self) -> Vec<u8>;
}

impl FormatResponse for TextResponse {
    fn format_response(&self) -> Vec<u8> {
        match self {
            TextResponse {
                status,
                headers,
                body,
            } => format!("{status}\r\n{headers}\r\n\r\n{body}")
                .as_bytes()
                .to_vec(),
        }
    }
}

// The content directory to server -should end in '/'
const SERVER_DIR: &str = "/home/jack/rust/webserver/src/pub/";
const STATUS_OK: &str = "HTTP/1.1 200 OK ";
const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 Not Found ";

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
    let buf_reader = BufReader::new(&mut stream);

    // A response to send if the resource isn't found
    let error_response = TextResponse {
        status: STATUS_NOT_FOUND.to_string(),
        headers: "".to_string(),
        body: "404 // Requested resource not found".to_string(),
    };

    // Split the request into parts and get the requested resource
    let request = buf_reader.lines().next().unwrap()?;
    let parts: Vec<_> = request.split(" ").collect();
    let resource = parts[1];

    let resolve_result = resolve_response(resource);

    let response = match resolve_result {
        Ok(response) => response,
        Err(_) => error_response.format_response(),
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
        return Err("Resource not found".into());
    }

    // Automatically resolve the root to index
    if path.as_os_str() == "/" {
        path = PathBuf::from("/index");
    }

    path.set_extension("md");

    let md_response = load_md(path)?;
    Ok(md_response.format_response())
}

/**
 * Loads a markdown file relative to the server directory
 */
fn load_md(path: PathBuf) -> Result<TextResponse> {
    let mut abs_path = PathBuf::from(SERVER_DIR);
    abs_path.push(path.strip_prefix("/")?.to_path_buf());

    println!("Loading {:#?}...", abs_path);

    //let file = File::open(&abs_path)?;
    //let reader = BufReader::new(file);

    let read_result = fs::read_to_string(abs_path)?;
    let markdown = parse_md(read_result);
    let length = markdown.len().to_string();

    let response = TextResponse {
        status: STATUS_OK.to_string(),
        headers: format!("Content-Length: {length}"),
        body: markdown,
    };

    return Ok(response);
}
