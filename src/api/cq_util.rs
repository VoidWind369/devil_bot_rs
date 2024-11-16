use serde::{Deserialize, Serialize};
use serde_json::Value;
use void_log::log_info;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RawMessageJson {
    pub app: Option<String>,
    #[serde(rename = "bizsrc")]
    pub biz_src: Option<String>,
    pub config: Option<Value>,
    pub extra: Option<Value>,
    pub meta: Option<Value>,
    pub prompt: Option<String>,
    pub ver: Option<String>,
    pub view: Option<String>,
}

impl RawMessageJson {
    pub fn format_json(raw_message: &str) -> serde_json::Result<RawMessageJson> {
        let str = raw_message.trim_start_matches("[CQ:json,data=").trim_end_matches("]");
        let json = str
            .replace("\\\"", "\"")
            .replace("&#44;", ",")
            .replace("&#91;", "[")
            .replace("&#93;", "]")
            .replace(";", ",");
        log_info!("To Json {}", &json);
        serde_json::from_str(&json)
    }
}
