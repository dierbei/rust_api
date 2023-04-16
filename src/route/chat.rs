use actix_web::{Responder, web};
use actix_web::web::Json;
use actix_web::get;
use crate::ApiResult;
use serde::{Deserialize, Serialize};
// use chatgpt::prelude::*;
// use chatgpt::types::CompletionResponse;

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
    // 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

    // // Getting the API key here
    // let key = String::from("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");
    //
    // /// Creating a new ChatGPT client.
    // /// Note that it requires an API key, and uses
    // /// tokens from your OpenAI API account balance.
    // let client = ChatGPT::new(key).unwrap();
    //
    // /// Sending a message and getting the completion
    // let response: CompletionResponse = client
    //     .send_message("Describe in five words the Rust programming language.")
    //     .await
    //     .unwrap();
    //
    // let content = response.message().content.clone();
    // Json(ApiResult::success(Some(content)))

    let resp = reqwest::get(format!("https://hub.onmicrosoft.cn/chat?q={}", info.q))
        .await
        .unwrap()
        .json::<Answer>()
        .await
        .unwrap();

    Json(ApiResult::success(Some(resp.answer)))
}