use std::fs;
use std::fs::File;
use std::sync::Arc;

use protobuf::Message;
use redis::Commands;

use crate::api;
use crate::db;
use crate::nouns;

pub struct Peer {
    pub user_id: Option<String>,
    pub db: Arc<db::Db>,
    pub redis: redis::Connection,
}

type PeerResult = Result<api::Response, String>;

pub fn new(db: Arc<db::Db>, redis: redis::Connection) -> Peer {
    Peer {
        user_id: None,
        db: db,
        redis: redis,
    }
}

impl Peer {
    pub fn command(&mut self, command: api::Commands) -> PeerResult {
        match command {
            api::Commands::Read(read) => read_op(&self.db, &read.params),
            api::Commands::Write(write) => write_op(&self.db, write.params),
            api::Commands::AuthBySession(auth) => self.auth_session_op(&auth.params),
            api::Commands::AuthByEmail(auth) => self.auth_email_op(&auth.params),
            _ => Err(format!("not implemented")),
        }
    }

    pub fn auth_session_op(&mut self, device_key: &api::DeviceKey) -> PeerResult {
        let session_json: String = self
            .redis
            .hget("session_keys", &device_key.device_key).unwrap();
        let session : api::Session = serde_json::from_str(&session_json).unwrap();
        Ok(api::Response {
            msg: "ok".to_string(),
            result: Some(api::Nouns::UserId(session.user_id)),
        })
    }

    pub fn auth_email_op(&self, _email: &api::Email) -> PeerResult {
        let user_id = api::Nouns::UserId("abc1".to_string());
        Ok(api::Response {
            msg: "ok".to_string(),
            result: Some(user_id),
        })
    }
}

pub fn read_op(db: &db::Db, query: &api::QueryById) -> PeerResult {
    let path = db.file_from_id(&query.id);
    match File::open(&path) {
        Ok(mut reader) => {
            let location = nouns::location::Location::parse_from_reader(&mut reader).unwrap();
            let noun: api::Nouns = api::Nouns::Location(location);
            Ok(api::Response {
                msg: "ok".to_string(),
                result: Some(noun),
            })
        }
        Err(e) => {
            println!("read_op: {} {}", path, e);
            Err(e.to_string())
        }
    }
}

pub fn write_op(db: &db::Db, location: nouns::location::Location) -> PeerResult {
    let id = db.write(&location);
    let path = db.file_from_id(&id);
    println!("write_op: {}", path);
    location
        .write_to_writer(&mut fs::File::create(path).unwrap())
        .unwrap();
    Ok(api::Response {
        msg: "ok".to_string(),
        result: None,
    })
}
