use serde::{Deserialize, Serialize};

use crate::nouns;

pub type Username = String;

#[derive(Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum Commands {
    #[serde(rename = "activity.get")]
    Read(ById),
    #[serde(rename = "activity.add")]
    Write(nouns::location::Location),
    #[serde(rename = "auth.session")]
    AuthBySession(DeviceKey),
    #[serde(rename = "auth.email")]
    AuthByEmail(Email),
    #[serde(rename = "hello")]
    Hello(ServerName),
    #[serde(rename = "user.detail")]
    UserDetail(Option<ByUsername>),
    #[serde(rename = "user.update")]
    UserUpdate(ByUpdatableUser),
    #[serde(rename = "stream.follow")]
    StreamFollow(ByFollowParams),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Nouns {
    Location(nouns::location::Location),
    User(nouns::user::User),
    Id(ById),
    None,
}

#[derive(Serialize, Deserialize)]
pub struct ByUpdatableUser {
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ByFollowParams{
    pub username: Option<String>,
    pub r#type: Option<String>,
    pub count: Option<i32>,
    pub order: Option<String>,
    pub follow: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct UserId {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ByUsername {
    pub username: String,
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
    pub device_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ById {
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
