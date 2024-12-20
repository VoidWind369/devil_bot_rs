use crate::util::Config;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Openid {
    id: Option<i64>,
    openid: Option<String>,
    name: Option<String>,
    qq: Option<String>,
}

impl Openid {
    pub async fn select_qq(qq: &str) -> Result<Self, Error> {
        let pool = Config::get().await.get_database().mysql_om().await;
        let sql = "select * from app_openid a, app_qq b where a.id = b.openid_id and b.qq = ?";
        sqlx::query_as::<_, Self>(sql)
            .bind(qq)
            .fetch_one(&pool)
            .await
    }
}

impl Display for Openid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "身份号：{}\n名称：{}\nQQ号：{}",
            &self.id.as_ref().unwrap(),
            &self.name.as_ref().unwrap(),
            &self.qq.as_ref().unwrap()
        )
    }
}
