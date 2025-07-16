use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use log::error;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod handler;
mod map_uri_to_file;
mod port;

#[tokio::main]
pub async fn run_dev_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = port::dev_port();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handler::handle_request))
                .await
            {
                error!("Error serving connection: {}", err);
            }
        });
    }
}
