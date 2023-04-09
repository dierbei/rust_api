use crate::ApiResult;
use actix_web::get;
use actix_web::web::Json;
use actix_web::Responder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TiktokBeautyRes {
    pub code: i32,
    pub msg: String,
    pub dsc: String,
    pub url: String,
    pub info: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TiktokBeautyResponse {
    pub dsc: String,
    pub url: String,
    pub info: String,
}

#[get("/tiktok_beauty")]
async fn tiktok_beauty() -> impl Responder {
    let resp = reqwest::get("https://zj.v.api.aa1.cn/api/video_dyv2/")
        .await
        .unwrap()
        .json::<TiktokBeautyRes>()
        .await
        .unwrap();

    Json(ApiResult::success(Some(TiktokBeautyResponse {
        dsc: resp.dsc,
        url: resp.url,
        info: resp.info,
    })))
}
