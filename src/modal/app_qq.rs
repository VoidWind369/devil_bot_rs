use crate::util::Config;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct AppQQ {
    pub id: Option<i64>,
    pub qq: Option<String>,
    pub openid_id: Option<i64>,
    pub openid: Option<String>,
    pub name: Option<String>,
}

impl AppQQ {
    pub async fn select(user: &str) -> Result<Self, Error> {
        let pool = Config::get().await.get_database().mysql_om().await;
        let sql = "select * from app_qq INNER JOIN app_openid on app_qq.openid_id=app_openid.id where app_qq.qq=?";
        sqlx::query_as::<_, Self>(sql).bind(user).fetch_one(&pool).await
    }
}
