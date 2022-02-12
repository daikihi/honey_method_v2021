use actix_web::{HttpServer, App, web};
use actix_files as fs;
use honey_method_v2021::presentations::controllers::{static_files_controller, prefecture_controller};

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/web/index.html", web::get().to(static_files_controller::index))
        .service(fs::Files::new("/web/static/scripts/", "./src/static/scripts/").show_files_listing())
        .service(prefecture_controller::get_all_prefectures)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}