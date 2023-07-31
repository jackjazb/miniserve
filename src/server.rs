use std::{
    error, fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    path::PathBuf,
};

use crate::{
    http::{ContentType, HTTPResponse, Status},
    router,
    site::Site,
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const GLOBAL_STYLE: &str = "
	body{
		font-family:helvetica;
	}
";

pub fn handle_connection(mut stream: TcpStream, site: &Site) -> Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);

    // Split the request into parts and get the requested resource
    let mut request = String::new();
    buf_reader.read_line(&mut request)?;

    let parts: Vec<_> = request.split(" ").collect();
    if parts.len() < 2 {
        return Err("Empty request.".into());
    }
    let resource = parts[1];
    let html = router::load_route(resource, &site.route_map);

    let response: Vec<u8> = match html {
        Some(s) => HTTPResponse::new(Status::Ok, Some(ContentType::HTML), &wrap_html(&s)),
        None => HTTPResponse::new(Status::NotFound, None, "404 // Resource not found"),
    }
    .into();

    stream.write_all(&response)?;
    Ok(())
}

fn wrap_html(html: &str) -> String {
    format!(
        "<head>
		<style>
			{GLOBAL_STYLE}
		</style>
	</head>
	<body>
		{html}
	</body>"
    )
}

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

    let read_result = fs::read_to_string(abs_path)?;

    let res = HTTPResponse::new(Status::Ok, content_type, &read_result);

    Ok(res)
}
