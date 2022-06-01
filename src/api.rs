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
    AuthBySession(AuthByDevice),
    #[serde(rename = "auth.email")]
    AuthByEmail(AuthByEmail),
    #[serde(rename = "hello")]
    Hello(Hello),
}

#[derive(Serialize, Deserialize)]
pub enum Nouns {
    Location(nouns::location::Location),
    UserId(UserId),
}

#[derive(Serialize, Deserialize)]
pub struct AuthByDevice {
    pub id: String,
    pub params: DeviceKey,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "user")]
pub struct UserId {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub device_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Hello {
    pub id: String,
    pub params: ServerDetail,
}

#[derive(Serialize, Deserialize)]
pub struct ServerDetail {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceKey {
    pub device_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthByEmail {
    pub id: String,
    pub params: Email,
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
pub enum Response {
    Error(String),
    Result(Nouns),
}

#[derive(Serialize, Deserialize)]
pub struct JsonRPCRequest {
    pub id: String,
    pub method: Commands,
}

#[derive(Serialize, Deserialize)]
pub struct JsonRPCResponse {
    pub id: String,
    pub result: Response,
}
