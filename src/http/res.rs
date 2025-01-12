use std::fs;
use super::headers::HTTPHeaders;

const SERVER_ERR_BODY: &str = "500, INTERNAL SERVER ERROR!";


pub struct ResponseBody {
    pub data: String,
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
            },
        }
    }

    pub fn build_response(&mut self) -> String {
        // TODO: dynamic summary
        let mut res = String::from("HTTP/1.1 ") + &self.status.to_string()+ " \r\n";

        self.headers
            .set_header_if_not_exists("Content-Length", &self.body.data.len().to_string());

        for header in self.headers.clone() {
            res.push_str(&format!("{}: {}\r\n", header.name(), header.value())[..]);
        }

        res.push_str("\r\n");
        res.push_str(&self.body.data);

        res
    }
}


pub fn send_file(filename: &str) -> ResponseBody {
    match fs::read_to_string(filename) {
        Ok(content) => ResponseBody { data: content },
        Err(err) => {
            eprintln!("Failed to read file {}: {}", filename, err);
             ResponseBody { data: SERVER_ERR_BODY.to_string() }
        }
    }
}

pub fn send_string(string: &str) -> ResponseBody {
    ResponseBody { data: string.to_string() }
}
