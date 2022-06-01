use serde::{Deserialize, Serialize};

use crate::nouns;

#[derive(Serialize, Deserialize)]
#[serde(tag="method", content="params")]
pub enum Commands {
    #[serde(rename = "activity.get")]
    Read(Read),
    #[serde(rename = "activity.add")]
    Write(Write),
    #[serde(rename = "auth.session")]
    AuthBySession(DeviceKey),
    #[serde(rename = "auth.email")]
    AuthByEmail(Email),
    #[serde(rename = "hello")]
    Hello(ServerName),
    #[serde(rename = "user.detail")]
    UserDetail
}

#[derive(Serialize, Deserialize)]
pub enum Nouns {
    Location(nouns::location::Location),
    #[serde(rename = "user")]
    UserId(UserId),
}

#[derive(Serialize, Deserialize)]
pub struct UserId {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub device_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServerName {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceKey {
    pub device_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub email: String,
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
#[serde(rename_all = "lowercase")]
pub enum Response {
    Error(String),
    Result(Nouns),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct JsonRPCRequest {
    pub id: String,
    #[serde(flatten)]
    pub method: Commands,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct JsonRPCResponse {
    pub id: String,
    #[serde(flatten)]
    pub result: Response,
}
