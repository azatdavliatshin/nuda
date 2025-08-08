use log::info;
use std::path::PathBuf;

pub fn map_uri_to_file(uri: &str) -> PathBuf {
    let mut path = uri.trim_start_matches('/').to_string();

    if path.is_empty() {
        path = "index.html".into();
    }

    let should_add_html = !path.contains('.') && !path.ends_with('/');

    if should_add_html {
        path.push_str(".html");
    }

    info!("Mapped URI to file: {path}");

    let try_public = PathBuf::from("public").join(&path);

    if try_public.exists() {
        try_public
    } else {
        PathBuf::from("pages").join(&path)
    }
}
