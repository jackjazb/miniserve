///
pub struct HTTPResponse {
    status: Status,
    status_string: String,
    headers: String,
    body: Vec<u8>,
}

pub enum Status {
    Ok,
    NotFound,
}

pub enum ContentType {
    Image,
    HTML,
}

impl HTTPResponse {
    pub fn new(status: Status, content_type: Option<ContentType>, body: &str) -> Self {
        let status_string = match status {
            Status::Ok => "HTTP/1.1 200 OK ",
            Status::NotFound => "HTTP/1.1 404 Not Found ",
        }
        .to_string();

        let content_type_string = match content_type {
            Some(ContentType::Image) => "image/*",
            Some(ContentType::HTML) => "text/html",
            None => "",
        }
        .to_string();

        let length = body.len().to_string();

        HTTPResponse {
            status,
            status_string,
            headers: format!("Content-Type: {content_type_string}\r\nContent-Length: {length}"),
            body: body.to_string().into_bytes(),
        }
    }
}

/// Convert the HTTPResponse into a byte array for transmission
impl Into<Vec<u8>> for HTTPResponse {
    fn into(self) -> Vec<u8> {
        let response = format!("{}\r\n{}\r\n\r\n", &self.status_string, &self.headers)
            .as_bytes()
            .to_vec();
        return [response, self.body].concat();
    }
}
