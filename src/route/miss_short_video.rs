use crate::ApiResult;
use actix_web::get;
use actix_web::web::Json;
use actix_web::Responder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MissShortVideoRes {
    pub error: i32,
    pub result: i32,
    pub mp4: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MissShortVideoResponse {
    pub mp4: String,
}

#[get("/miss_short_video")]
async fn miss_short_video() -> impl Responder {
    let resp = reqwest::get("https://v.api.aa1.cn/api/api-dy-girl/index.php?aa1=json")
        .await
        .unwrap()
        .json::<MissShortVideoRes>()
        .await
        .unwrap();

    Json(ApiResult::success(Some(MissShortVideoResponse {
        mp4: resp.mp4,
    })))
}
