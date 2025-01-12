use super::{res::{send_string, Response, ResponseBody}, Request};

pub type RouteHandler = fn(Request, &mut Response) -> ResponseBody;

pub fn not_found(_req: Request, res: &mut Response) -> ResponseBody {
    res.status = 404; 
    send_string("404, PAGE NOT FOUND")
}

pub fn empty_response(_req: Request, _res: &mut Response) -> ResponseBody {
    ResponseBody { data: String::new() }
}

