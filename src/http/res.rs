use super::headers::HTTPHeaders;
use std::fs;

const SERVER_ERR_BODY: &str = "500, INTERNAL SERVER ERROR!";

pub struct ResponseBody {
    pub data: String,
    content_type: Option<String>,
}

impl ResponseBody {
    pub fn build(data: String, content_type: Option<String>) -> ResponseBody {
        ResponseBody { data, content_type }
    }

    pub fn from_data(data: String) -> ResponseBody {
        ResponseBody::build(data, None)
    }

    pub fn content_type(&self) -> &str {
        match &self.content_type {
            Some(content_type) => content_type,
            None => "text/plain",
        }
    }

    pub fn set_content_type(&mut self, content_type: String) -> () {
        self.content_type = Some(content_type);
    }
}

pub struct Response {
    pub status: u16,
    pub headers: HTTPHeaders,
    pub body: ResponseBody,
}

impl Response {
    pub fn new() -> Response {
        Response {
            status: 200,
            headers: HTTPHeaders::new(),
            body: ResponseBody {
                data: "".to_string(),
                content_type: None,
            },
        }
    }

    pub fn build_response(&mut self) -> String {
        self.headers
            .set_header_if_not_exists("Content-Length", &self.body.data.len().to_string());

        match &self.body.content_type {
            Some(content_type) => self.headers.set_header_if_not_exists("Content-Type", content_type),
            None => ()
        }
        // if let Some(content_type) = &self.body.content_type {
        //     self.headers
        //         .set_header_if_not_exists("Content-Type", &content_type);
        // }

        let mut res = String::from("HTTP/1.1 ") + &self.status.to_string() + " \r\n";

        for header in self.headers.clone() {
            res.push_str(&format!("{}: {}\r\n", header.name(), header.value())[..]);
        }

        res.push_str("\r\n");
        res.push_str(&self.body.data);

        res
    }
}

pub fn send_file(filename: &str) -> ResponseBody {
    let content_type = match filename.rsplit_once(".") {
        Some(ext) => match ext.1 {
            "json" => "application/json",
            "css" => "text/css",
            "html" => "text/html",
            "js" => "text/javascript",
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            _ => "text/plain",
        },
        None => "text/plain",
    }
    .to_string();

    match fs::read_to_string(filename) {
        Ok(content) => ResponseBody::build(content, Some(content_type)),
        Err(err) => {
            eprintln!("Failed to read file {}: {}", filename, err);
            ResponseBody::from_data(SERVER_ERR_BODY.to_string())
        }
    }
}

pub fn send_string(string: &str) -> ResponseBody {
    ResponseBody::from_data(string.to_string())
}
