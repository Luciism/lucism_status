use std::path::Path;

use super::{
    res::{send_string, Response, ResponseBody},
    send_file, Request,
};

pub type RouteHandler = fn(Request, &mut Response) -> ResponseBody;

mod paths {
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};

    fn sanitize_filename(filename: &str) -> String {
        filename
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '.' || c == '_' || c == '-' || c == '/' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }

    fn is_safe_path(base_dir: &Path, user_path: &Path) -> io::Result<bool> {
        let abs_base_dir = fs::canonicalize(base_dir)?;
        let abs_user_path = fs::canonicalize(user_path)?;
        Ok(abs_user_path.starts_with(&abs_base_dir))
    }

    pub fn get_safe_path(base_dir: &Path, user_path: &Path) -> io::Result<PathBuf> {
        let sanitized_path = PathBuf::from(sanitize_filename(user_path.to_str().unwrap()));
        if is_safe_path(base_dir, &sanitized_path)? {
            Ok(fs::canonicalize(sanitized_path)?)
        } else {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Unsafe file path detected!",
            ))
        }
    }
}

pub fn not_found(_req: Request, res: &mut Response) -> ResponseBody {
    res.status = 404;
    send_string("404, PAGE NOT FOUND")
}

pub fn empty_response(_req: Request, _res: &mut Response) -> ResponseBody {
    ResponseBody::from_data(String::new())
}

pub fn static_file(req: Request, res: &mut Response) -> ResponseBody {
    let mut filepath = req.path.clone();
    filepath.remove(0); // Remove leading slash
    req.path.clone().remove(0);

    match paths::get_safe_path(Path::new("static/"), Path::new(&filepath)) {
        Ok(filepath) => match filepath.to_str() {
            Some(filepath) => return send_file(&filepath),
            None => res.status = 404
        },
        Err(_) => res.status = 404
    };

    return empty_response(req, res);
}

pub fn default_route_handler(request: &Request) -> Option<RouteHandler> {
    if request.path.starts_with("/static/") {
        return Some(static_file);
    }

    None
}
