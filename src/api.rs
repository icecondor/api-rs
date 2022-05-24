use serde::{Deserialize, Serialize};

use crate::nouns;

#[derive(Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum Commands {
    Read(Read),
    Write(Write),
    #[serde(rename = "auth.session")]
    Auth(Auth),
}

#[derive(Serialize, Deserialize)]
pub enum Nouns {
    Location(nouns::location::Location),
    UserId(String),
}

#[derive(Serialize, Deserialize)]
pub struct Auth {
    pub device_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Write {
    pub id: String,
    pub params: nouns::location::Location,
}

#[derive(Serialize, Deserialize)]
pub struct Read {
    pub id: String,
    pub params: QueryById,
}

#[derive(Serialize, Deserialize)]
pub struct QueryById {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noun: Option<Nouns>,
}
