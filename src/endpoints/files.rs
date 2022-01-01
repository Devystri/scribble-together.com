use std::path::PathBuf;
use actix_files::NamedFile;

use actix_web::{get, web, App, HttpRequest, HttpServer, Responder, Error};

/// Served files must be placed under a `files` directory located beside the server program.  
/// The file `files/index.html` will be served at the root.  
/// Directories are not supported.  
/// A `.html` extension will be added if no extension is provided. Thus, `/test` will serve the `/files/test.html`.  
#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> Result<NamedFile, Error> {
    let mut path = PathBuf::new();
    path.push("frontend/");
    let requested_path = req.match_info().get("filename").unwrap();
    match requested_path {
        "" | "/" => path.push("index.html"),
        _ => path.push(requested_path),
    }

    // If there is no extension, html is inferred
    if path.extension().is_none() {
        path.push("index.html");
    }
    println!("{:?}", path);
    let file = NamedFile::open(path)?;
    Ok(file)
}