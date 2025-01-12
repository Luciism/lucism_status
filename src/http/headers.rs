#[derive(Debug, Clone)]
pub struct HTTPHeader {
    name: String,
    value: String,
}

impl HTTPHeader {
    pub fn build(name: &str, value: &str) -> HTTPHeader {
        HTTPHeader {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    pub fn name(&self) -> &String {&self.name}
    pub fn value(&self) -> &String {&self.value}

    pub fn set_value(&mut self, value: &str) -> () {
        self.value = value.to_string();
    }
}

#[derive(Debug, Clone)]
pub struct HTTPHeaders {
    headers: Vec<HTTPHeader>,
}

impl HTTPHeaders {
    pub fn new() -> HTTPHeaders {
        HTTPHeaders { headers: vec![] }
    }

    pub fn from_headers(headers: Vec<(String, String)>) -> HTTPHeaders {
        let results = headers
            .iter()
            .map(|header| HTTPHeader::build(&header.0, &header.1))
            .collect();
        HTTPHeaders { headers: results }
    }

    pub fn get_header(&mut self, name: &str) -> Option<&mut HTTPHeader> {
        self.headers.iter_mut().find(|header| header.name == name)
    }

    pub fn set_header(&mut self, name: &str, value: &str) -> () {
        match self.get_header(name) {
            Some(header) => header.set_value(value),
            None => self.headers.push(HTTPHeader::build(name, value))
        }
    }

    pub fn set_header_if_not_exists(&mut self, name: &str, value: &str) -> () {
        match self.get_header(name) {
            Some(_) => {}
            None => self.set_header(name, value),
        }
    }
}

// impl Iterator for HTTPHeaders {
//     type Item = HTTPHeader;

//     fn next(&mut self) -> Option<Self::Item> {
//         // <Vec<HTTPHeader> as Clone>::clone(&self.headers).into_iter().next()
//         // self.headers.iter().next().cloned()
//     }
// }


impl IntoIterator for HTTPHeaders {
    type Item = HTTPHeader;
    type IntoIter = std::vec::IntoIter<HTTPHeader>;

    fn into_iter(self) -> Self::IntoIter {
        self.headers.into_iter()
    }
}
