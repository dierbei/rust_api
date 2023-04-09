use crate::ApiResult;
use actix_web::get;
use actix_web::web::Json;
use actix_web::Responder;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use regex::Regex;
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
    // 定义需要编码的字符集，这里包含了所有控制字符和空格字符
    const FRAGMENT: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'|')
        .add(b'}');

    let resp = reqwest::get("https://v.api.aa1.cn/api/api-dy-girl/index.php?aa1=json")
        .await
        .unwrap()
        .json::<MissShortVideoRes>()
        .await
        .unwrap();

    let re = Regex::new(r"/([^/]+)\.mp4").unwrap();
    let cap = re.captures(&resp.mp4).unwrap();
    let filename = cap.get(1).unwrap().as_str();
    let output_str = utf8_percent_encode(filename, FRAGMENT).to_string();

    // https://v.api.aa1.cn/api/api-dy-girl/video/0084.mp434%E5%8C%85%20api.aa1.cn%20%20%E5%85%8D%E8%B4%B9%E8%A7%86%E9%A2%91API.mp4
    Json(ApiResult::success(Some(MissShortVideoResponse {
        mp4: format!(
            "https://v.api.aa1.cn/api/api-dy-girl/video/{}.mp4",
            output_str
        ),
        // mp4: resp.mp4,
    })))
}
