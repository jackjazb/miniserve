use std::{
    error,
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

// override result type
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

const SERVER_DIR: &str = "/home/jack/rust/webserver/src/";
const STATUS_OK: &str = "HTTP/1.1 200 OK ";
const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 Not Found ";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // Split the request into parts and get the requested resource
    let request = buf_reader.lines().next().unwrap().unwrap();
    let parts: Vec<_> = request.split(" ").collect();
    let resource = parts[1];

    let resolve_result = resolve_response(resource);

    let response = match resolve_result {
        Ok(response) => response,
        Err(_) => ("Resource not found!").as_bytes().to_vec(),
    };

    stream.write_all(&response).unwrap();
}

fn resolve_response(requested_resource: &str) -> Result<Vec<u8>> {
    let error_response = TextResponse {
        status: STATUS_NOT_FOUND.to_string(),
        headers: "".to_string(),
        body: "404 // Requested resource not found".to_string(),
    };
    let mut path = PathBuf::from(requested_resource);

    let has_extension = path.extension().is_some();

    // If an extension has been provided, load that resource if available
    if has_extension {
        return Ok(error_response.format_response());
    }
    if path.as_os_str() == "/" {
        path = PathBuf::from("/index");
    }

    path.set_extension("md");
    let load_result = load_md(path);
    let md_response = match load_result {
        Ok(res) => res,
        Err(_) => error_response,
    };

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
    let length = String::from(read_result.len().to_string());

    let response = TextResponse {
        status: STATUS_OK.to_string(),
        headers: format!("Content-Length: {length}"),
        body: read_result,
    };

    return Ok(response);
}
