use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ApiResult<T: Serialize> {
    pub ok: bool,
    pub err: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
    pub fn success(r: Option<T>) -> ApiResult<T> {
        ApiResult {
            ok: true,
            err: None,
            data: r,
        }
    }
    pub fn error<E: ToString>(err: E) -> ApiResult<T> {
        ApiResult {
            ok: false,
            err: Some(err.to_string()),
            data: None,
        }
    }
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    let data = "Hello, world!";
    Json(ApiResult::success(Some(data)))
}

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
        info: resp.info
    })))
}

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

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);
        App::new().wrap(cors).service(
            web::scope("/api")
                .service(health_check)
                .service(tiktok_beauty)
                .service(history_today),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
