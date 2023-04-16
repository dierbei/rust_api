use actix_web::{Responder, web};
use actix_web::web::Json;
use actix_web::get;
use crate::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    q: String,
}

#[derive(Serialize, Deserialize)]
struct Answer {
    answer: String,
}

#[get("/chat")]
async fn chat(info: web::Query<Info>) -> impl Responder {
    let resp = reqwest::get(format!("https://hub.onmicrosoft.cn/chat?q={}", info.q))
        .await
        .unwrap()
        .json::<Answer>()
        .await
        .unwrap();

    Json(ApiResult::success(Some(resp.answer)))
}