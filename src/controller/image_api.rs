use std::io::Cursor;
use crate::om_api::record::Record;
use axum::extract::Path;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use image::ImageFormat;

async fn get_record(Path(tag): Path<String>) -> impl IntoResponse {
    let data = Record::new_json(&tag, '0').await.list_img(50).await;

    // 创建一个内存中的缓冲区（Cursor<Vec<u8>）
    let mut buffer = Cursor::new(Vec::new());
    // 将图像数据写入缓冲区
    data.write_to(&mut buffer, ImageFormat::Png).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("image/png"),
    );
    (headers, buffer.into_inner()).into_response()
}

pub fn router(app: Router) -> Router {
    let img = Router::new()
        .route("/get_record/{tag}", get(get_record));
    app.nest("/img", img)
}
