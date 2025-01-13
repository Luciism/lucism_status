use std::{
    io::{prelude::BufRead, BufReader, Read},
    net::TcpStream,
};

use super::{
    headers::{HTTPHeader, HTTPHeaders},
    status::Status,
};

const MAX_CONTENT: usize = 2_097_152; // 2MB

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    GET,
    POST,
}

impl RequestMethod {
    pub fn from_string(method: &str) -> Option<RequestMethod> {
        match method {
            "GET" => Some(RequestMethod::GET),
            "POST" => Some(RequestMethod::POST),
            _ => None,
        }
    }

    pub fn supports_body(&self) -> bool {
        match self {
            RequestMethod::POST => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct RequestBody {
    data: String,
}

impl RequestBody {
    pub fn from(data: String) -> RequestBody {
        RequestBody { data }
    }

    pub fn content(&self) -> &String {
        &self.data
    }
}

pub struct Request {
    pub method: RequestMethod,
    pub path: String,
    pub headers: HTTPHeaders,
    pub body: Option<RequestBody>,
}

impl Request {
    pub fn build(method: RequestMethod, path: String, body: Option<String>) -> Request {
        Request {
            method,
            path,
            headers: HTTPHeaders::new(),
            body: match body {
                Some(body) => Some(RequestBody::from(body)),
                None => None,
            },
        }
    }

    pub fn default() -> Request {
        Request::build(RequestMethod::GET, "/".to_string(), None)
    }
}

#[derive(Clone, Debug)]
pub struct HttpError {
    pub status: u16,
    pub cause: String,
}

fn err_bad_request() -> HttpError {
    return HttpError {
        status: 400,
        cause: "Bad Request".to_string(),
    };
}

fn parse_request_line<'a>(
    request_line: &'a String,
) -> Result<(RequestMethod, String, String), HttpError> {
    let split_line: Vec<&str> = request_line.split(' ').collect();

    if split_line.len() != 3 {
        return Err(err_bad_request());
    }

    let method = split_line[0].to_string();
    let path = split_line[1].to_string();
    let http_ver = split_line[2].to_string();

    // Validate method
    let method = match RequestMethod::from_string(&method) {
        Some(method) => method,
        None => {
            return Err(HttpError {
                status: Status::method_not_allowed(),
                cause: "Method Not Allowed".to_string(),
            })
        }
    };

    Ok((method, path, http_ver))
}

fn split_header_line(header_line: &String) -> Result<(String, String), HttpError> {
    let split: Vec<&str> = header_line.splitn(2, ": ").collect();
    if split.len() != 2 {
        return Err(err_bad_request());
    }

    Ok((split[0].to_string(), split[1].to_string()))
}

fn parse_request_headers(http_request_headers: &[&str]) -> Result<HTTPHeaders, HttpError> {
    let mut header_lines: Vec<(String, String)> = vec![];

    for line in http_request_headers {
        header_lines.push(split_header_line(&line.to_string())?);
    }

    Ok(HTTPHeaders::from_headers(header_lines))
}

fn read_until_newline(buf_reader: &mut BufReader<&TcpStream>) -> Result<String, HttpError> {
    let mut result = String::new();

    loop {
        let mut chunk = String::new();
        match buf_reader.read_line(&mut chunk) {
            Ok(0) => break,
            Ok(_) => {
                if chunk.trim().is_empty() {
                    break;
                }
                result.push_str(&chunk);
            }
            Err(_) => return Err(err_bad_request()),
        }
    }

    return Ok(result.trim().to_string());
}

fn read_body(
    mut buf_reader: BufReader<&TcpStream>,
    content_length_header: Option<&mut HTTPHeader>,
) -> Result<Option<String>, HttpError> {
    Ok(match content_length_header {
        Some(content_length) => {
            let content_length: usize = match content_length.value().parse() {
                Ok(content_length) => content_length,
                Err(_) => return Err(err_bad_request()),
            };
            if content_length > MAX_CONTENT {
                return Err(err_bad_request());
            }

            let mut body = Vec::with_capacity(content_length);
            body.resize(content_length, 0);

            let bytes_read = buf_reader.read(&mut body).map_err(|_| err_bad_request())?;
            if bytes_read != content_length {
                println!(
                    "Wrong length: got {}, expected {}",
                    bytes_read, content_length
                );
                return Err(err_bad_request());
            }

            let body = String::from_utf8(body).map_err(|_| err_bad_request())?;
            Some(body)
        }
        None => None,
    })
}

pub fn build_request<'a>(stream: &'a TcpStream) -> Result<Request, HttpError> {
    let mut buf_reader = BufReader::new(stream);

    let req_headers = read_until_newline(&mut buf_reader)?;
    let req_headers: Vec<&str> = req_headers.split("\r\n").collect();

    let request_line = req_headers[0];
    let (method, path, _http_ver) = parse_request_line(&request_line.to_string())?;

    let mut headers = parse_request_headers(&req_headers[1..])?;

    let body = match method.supports_body() {
        true => read_body(buf_reader, headers.get_header("Content-Length"))?,
        false => None,
    };

    let mut req = Request::build(method, path, body);
    req.headers = headers;

    Ok(req)
}
