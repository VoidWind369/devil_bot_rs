use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message<'a> {
    name: &'a str,
    msg: &'a str,
    create_time: NaiveDateTime,
}

impl<'a> Message<'a> {
    pub fn new(name: &'a str, msg: &'a str) -> Message<'a> {
        Self {
            name,
            msg,
            create_time: Local::now().naive_local(),
        }
    }
}