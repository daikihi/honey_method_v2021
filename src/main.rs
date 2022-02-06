use actix_web::{HttpServer, App, web};
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use actix_files as fs;
use std::path::PathBuf;



async fn index(_: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./src/static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/web/index.html", web::get().to(index))
        .service(fs::Files::new("/web/static/scripts/", "./src/static/scripts/").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
