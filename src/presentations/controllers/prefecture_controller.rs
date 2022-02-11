
use actix_web::HttpResponse;
use actix_web::{get};

use crate::domain::use_case::prefectures_use_case::get_all_prefectures_use_case;


#[get("/api/prefectures")]
pub async fn get_all_prefectures()  -> HttpResponse {
    let result = get_all_prefectures_use_case();
    HttpResponse::Ok().json(result.await)
}
