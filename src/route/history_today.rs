use crate::ApiResult;
use actix_web::get;
use actix_web::web::Json;
use actix_web::Responder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct HistoryTodayRes {
    pub code: i32,
    pub month: String,
    pub day: String,
    pub data: Vec<HistoryTodayData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HistoryTodayData {
    pub year: i32,
    pub title: String,
    pub link: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[get("/history_today")]
async fn history_today() -> impl Responder {
    let resp = reqwest::get("https://query.asilu.com/today/list/")
        .await
        .unwrap()
        .json::<HistoryTodayRes>()
        .await
        .unwrap();

    Json(ApiResult::success(Some(resp)))
}
