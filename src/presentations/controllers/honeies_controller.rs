use actix_web::{HttpResponse, Responder};
use actix_web::get;
use crate::domain::use_case::honey_master_use_case;

#[get("/api/honeies")]
pub async fn get_all_honeies() -> impl Responder{
    let res = honey_master_use_case::get_all_honeies().await;
    HttpResponse::Ok().json(res)
}