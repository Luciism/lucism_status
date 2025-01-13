use lucism_status::http;

fn main() -> ! {
    let mut server = http::HTTPServer::new("127.0.0.1", 7878);

    server.post("/", |req: http::Request, _res: &mut http::Response| {
        println!("{:#?}", req.body);
        http::send_string("Hello")
    });

    server.get("/", |_req: http::Request, _res: &mut http::Response| {
        http::send_file("templates/index.html")
    });

    server.start();
}

