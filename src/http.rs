const HTTP_STATUS_200: &str = "HTTP/1.1 200 OK ";
const HTTP_STATUS_404: &str = "HTTP/1.1 404 Not Found ";
pub struct HTTPResponse {
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
    pub fn okay(content_type: Option<ContentType>, body: Vec<u8>) -> Self {
        Self::new(Status::Ok, content_type, body)
    }

    pub fn not_found(content_type: Option<ContentType>, body: Vec<u8>) -> Self {
        Self::new(Status::NotFound, content_type, body)
    }

    pub fn new(status: Status, content_type: Option<ContentType>, body: Vec<u8>) -> Self {
        let status_string = match status {
            Status::Ok => HTTP_STATUS_200,
            Status::NotFound => HTTP_STATUS_404,
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
            status_string,
            headers: format!("Content-Type: {content_type_string}\r\nContent-Length: {length}"),
            body,
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
