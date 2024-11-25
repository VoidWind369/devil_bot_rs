use crate::util::ConfigDatabase;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::{Connection, MySql, Pool};
use std::str::FromStr;

pub mod app_qq;

impl ConfigDatabase {
    pub async fn mysql_om(self) -> Pool<MySql> {
        let option = MySqlConnectOptions::from_str(&self.url.unwrap())
            .unwrap().username(&self.username.unwrap())
            .password(&self.password.unwrap());
        sqlx::MySqlPool::connect_with(option).await.unwrap()
    }
}