use crate::util::Config;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct AppAdminPower {
    id: i64,
    admin_id: i64,
    #[sqlx(rename = "adminpower_id")]
    admin_power_id: i64,
}

impl AppAdminPower {

    pub async fn select_admin_id(admin_id: i64, admin_power_id: i64) -> Result<Self, Error> {
        let pool = Config::get().await.get_database().mysql_om().await;
        let sql = "SELECT * FROM app_admin_admin_power WHERE admin_id = ? and adminpower_id = ?";
        sqlx::query_as::<_, Self>(sql)
            .bind(admin_id)
            .bind(admin_power_id)
            .fetch_one(&pool)
            .await
    }
}
