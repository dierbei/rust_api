use crate::ApiResult;
use actix_web::get;
use actix_web::web::Json;
use actix_web::Responder;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    let data = "Hello, world!";
    Json(ApiResult::success(Some(data)))
}
