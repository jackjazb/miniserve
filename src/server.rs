use std::{
    error, fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    path::{Path, PathBuf},
};

use crate::{
    http::{ContentType, HTTPResponse, Status},
    renderer,
    router::{self, RouteMap},
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn handle_connection(mut stream: TcpStream, route_map: &RouteMap) -> Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);

    // Split the request into parts and get the requested resource
    let mut request = String::new();
    buf_reader.read_line(&mut request)?;

    let parts: Vec<_> = request.split(" ").collect();
    if parts.len() < 2 {
        return Err("Empty request.".into());
    }
    let path = parts[1];

    if let Some(_) = Path::new(path).extension() {
        if let Ok(res) = load_file_response(PathBuf::from(path)) {
            let bytes: Vec<u8> = res.into();
            stream.write_all(&bytes)?;
            return Ok(());
        }
    }

    // Extract the referenced Route from the map, and render it if it exists
    let rendered: Option<String> = router::parse_route(path, &route_map)
        .and_then(|route| Some(renderer::render_route(&route, &path, &route_map)));

    let response: Vec<u8> = match rendered {
        Some(body) => HTTPResponse::okay(Some(ContentType::HTML), body.to_string().into_bytes()),
        None => HTTPResponse::not_found(None, "404 // Resource not found".to_string().into_bytes()),
    }
    .into();

    stream.write_all(&response)?;
    Ok(())
}

// TODO
fn load_file_response(path: PathBuf) -> Result<HTTPResponse> {
    let extension = path.extension();
    // Match different extensions for content type here
    let content_type = match extension {
        Some(os_str) => match os_str.to_str() {
            Some("ico") => Some(ContentType::Image),
            Some("png") => Some(ContentType::Image),
            Some(&_) => Some(ContentType::HTML),
            None => None,
        },
        None => None,
    };

    let mut abs_path = PathBuf::from("./site");
    abs_path.push(path.strip_prefix("/")?.to_path_buf());
    let read_result = fs::read(abs_path)?;

    let res = HTTPResponse::okay(content_type, read_result);

    Ok(res)
}
