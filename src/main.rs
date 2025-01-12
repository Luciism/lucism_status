use lucism_status::http;

fn main() -> ! {
    let mut server = http::HTTPServer::new("127.0.0.1", 7878);

    server.get("/", |req: http::Request, res: &mut http::Response| {
        res.headers.set_header("Yourmom", "abc123");
        println!("{:#?}", req.body);
        http::send_file("templates/hello.html")
    });


    server.post("/", |req: http::Request, res: &mut http::Response| {
        res.headers.set_header("Yourmom", "abc123");
        println!("{:#?}", req.body);
        http::send_file("templates/hello.html")
    });

    server.start();
}

