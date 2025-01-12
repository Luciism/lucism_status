pub struct Status;

#[allow(dead_code)]
impl Status {
    pub fn success() -> u16 { 200 }
    pub fn created() -> u16 { 201 }
    pub fn accepted() -> u16 { 202 }
    pub fn no_content() -> u16 { 204 }
    pub fn bad_request() -> u16 { 400 }
    pub fn unauthorized() -> u16 { 401 }
    pub fn forbidden() -> u16 { 403 }
    pub fn not_found() -> u16 { 404 }
    pub fn method_not_allowed() -> u16 { 405 }
    pub fn internal_server_error() -> u16 { 500 }
    pub fn not_implemented() -> u16 { 501 }
    pub fn bad_gateway() -> u16 { 502 }
    pub fn service_unavailable() -> u16 { 503 }
}

