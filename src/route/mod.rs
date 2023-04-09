pub mod health_check;
pub mod history_today;
pub mod miss_short_video;
pub mod tiktok_beauty;

pub use health_check::*;
pub use history_today::*;
pub use miss_short_video::*;
use serde::Serialize;
pub use tiktok_beauty::*;

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
