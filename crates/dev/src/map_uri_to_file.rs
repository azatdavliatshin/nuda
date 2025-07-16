use std::path::PathBuf;

pub fn map_uri_to_file(uri: &str) -> PathBuf {
    let path = if uri == "/" {
        "index.html"
    } else {
        uri.trim_start_matches('/')
    };

    let try_public = PathBuf::from("public").join(path);

    if try_public.exists() {
        try_public
    } else {
        PathBuf::from("pages").join(path)
    }
}
