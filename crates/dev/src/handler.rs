use crate::map_uri_to_file;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use log::{error, info};
use std::convert::Infallible;
use tokio::fs;

pub async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let method = req.method();
    let uri_path = req.uri().path();
    info!("→ {} {}", method, uri_path);

    if method != hyper::Method::GET && method != hyper::Method::HEAD {
        // set 405 Method Not Allowed
        return Ok(Response::builder()
            .status(405)
            .body(Full::new(Bytes::from("Method Not Allowed")))
            .unwrap());
    }

    let path = map_uri_to_file::map_uri_to_file(uri_path);

    match fs::read(&path).await {
        Ok(contents) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            let res = Response::builder()
                .status(200)
                .header("Content-Type", mime.as_ref())
                .body(Full::new(Bytes::from(contents)))
                .unwrap();
            Ok(res)
        }
        Err(_) => {
            error!("⚠️  Not found: {}", path.display());
            return Ok(Response::builder()
                .status(404)
                .body(Full::new(Bytes::from("Not Found")))
                .unwrap());
        }
    }
}
