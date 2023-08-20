use std::{
    collections::BTreeMap,
    error, fs,
    io::{self, BufRead, BufReader, ErrorKind},
    net::TcpStream,
    ops::Add,
    path::{Path, PathBuf},
};

use crate::{
    http::{ContentType, HTTPResponse},
    renderer,
    router::{self, Route, RouteMap},
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct Server {
    route_map: RouteMap,
}

impl Server {
    pub fn new(path: &str) -> Option<Self> {
        if let Ok(route_map) = load_routes_recursive(path) {
            if !route_map.contains_key("index") {
                println!("Could not find index.md in the specified directory");
                return None;
            }
            return Some(Server { route_map });
        }
        None
    }

    /// Reads an HTTP request from a TCP stream, and writes an HTTP response of some kind
    pub fn handle_connection(&self, mut stream: &TcpStream) -> Result<HTTPResponse> {
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
            return Self::load_file_response(PathBuf::from(path));
        }

        // Extract the referenced Route from the map, and render it if it exists
        let rendered: Option<String> = router::parse_route(path, &self.route_map)
            .and_then(|route| Some(renderer::render_route(&route, &path, &self.route_map)));

        let response = match rendered {
            Some(body) => {
                HTTPResponse::okay(Some(ContentType::HTML), body.to_string().into_bytes())
            }
            None => {
                HTTPResponse::not_found(None, "404 // Resource not found".to_string().into_bytes())
            }
        };
        Ok(response)
    }

    /// Loads a file from disk and wraps it in an HTTPResponse
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
}

/// Recursively loads site markup from a specified directory into a RouteMap.
fn load_routes_recursive(path: &str) -> Result<RouteMap> {
    let mut route_map: RouteMap = BTreeMap::new();

    for entry in fs::read_dir(path)? {
        let item_path = entry?.path();
        let file_name = item_path
            .file_stem()
            .and_then(|p| p.to_str())
            .ok_or(io::Error::new(ErrorKind::Other, "Failed to resolve path"))?;

        let metadata = fs::metadata(&item_path)?;

        // If the resource is a directory, recursively load sub routes.
        // Not that example.com/posts/
        match metadata.is_dir() {
            true => {
                let sub_path = String::from(path).add("/").add(file_name);
                if let Ok(sub_routes) = load_routes_recursive(&sub_path) {
                    route_map.insert(file_name.to_string(), Route::Directory(sub_routes));
                }
            }
            false => {
                let ext = item_path
                    .extension()
                    .and_then(|s| s.to_str())
                    .ok_or(io::Error::new(
                        ErrorKind::Other,
                        "Failed to resolve resource extension",
                    ))?;

                if ext == "md" {
                    let markdown = fs::read_to_string(&item_path)?;
                    let html = markdown::to_html(&markdown);
                    // let created_at = metadata.created()?;
                    route_map.insert(file_name.to_string(), Route::Page(html));
                }
            }
        }
    }

    Ok(route_map)
}

#[cfg(test)]
mod tests {}
