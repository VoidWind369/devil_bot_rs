use crate::om_api::record::Record;
use axum::extract::Path;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

async fn get_record(Path(tag): Path<String>) -> impl IntoResponse {
    let data = Record::new(&tag, 0).await.list_img(50).await.into_bytes();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("image/png"),
    );
    (headers, data).into_response()
}

pub fn router(app: Router) -> Router {
    let img = Router::new()
        .route("/get_record", get(get_record));
    app.nest("/img", img)
}
