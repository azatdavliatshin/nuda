use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use shared::config::load_config;

#[tokio::main]
pub async fn run_dev_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = dev_port();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection: {}", err);
            }
        });
    }
}

fn dev_port() -> u16 {
    if let Ok(port_str) = std::env::var("NUDA_DEV_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            return port;
        }
    }

    if let Some(cfg) = load_config() {
        if let Some(dev) = cfg.dev {
            return dev.port;
        }
    }

    3000
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
