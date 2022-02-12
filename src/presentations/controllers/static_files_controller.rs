use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

pub async fn index(_: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./src/static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
