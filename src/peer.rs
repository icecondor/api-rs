use std::fs;
use std::fs::File;
use std::sync::Arc;
use protobuf::Message;

use serde_json;

use crate::api;
use crate::db;
use crate::nouns;

pub struct Peer {
    pub user_id: Option<String>,
    pub db: Arc<db::Db>,
}

type PeerResult = Result<api::Response, String>;

pub fn new(db: Arc<db::Db>) -> Peer {
    Peer {
        user_id: None,
        db: db,
    }
}

impl Peer {
    pub fn command(&self, line: &str) -> PeerResult {
        let command: api::Commands = serde_json::from_str(&line).unwrap();
        println!("{}", serde_json::to_string(&command).unwrap());
        self.do_command(command)
    }

    pub fn do_command(&self, command: api::Commands) -> PeerResult {
        match command {
            api::Commands::Read(read) => read_op(&self.db, &read.params),
            api::Commands::Write(write) => write_op(&self.db, write.params),
            api::Commands::AuthBySession(auth) => auth_device_op(&self.db, &auth.params),
            api::Commands::AuthByEmail(auth) => auth_email_op(&self.db, &auth.params),
            //_ => Err(format!("not implemented"))
        }
    }
}

pub fn auth_device_op(db: &db::Db, device_key: &api::DeviceId) -> PeerResult {
    let user_id = api::Nouns::UserId("abc1".to_string());
    Ok(api::Response {
        msg: "ok".to_string(),
        noun: Some(user_id),
    })
}

pub fn auth_email_op(db: &db::Db, email: &api::Email) -> PeerResult {
    let user_id = api::Nouns::UserId("abc1".to_string());
    Ok(api::Response {
        msg: "ok".to_string(),
        noun: Some(user_id),
    })
}

pub fn read_op(db: &db::Db, query: &api::QueryById) -> PeerResult {
    let path = db.file_from_id(&query.id);
    match File::open(&path) {
        Ok(mut reader) => {
            let location = nouns::location::Location::parse_from_reader(&mut reader).unwrap();
            let noun: api::Nouns = api::Nouns::Location(location);
            Ok(api::Response {
                msg: "ok".to_string(),
                noun: Some(noun),
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
    let json = serde_json::to_string(&location).unwrap();
    println!("write_op: {} {}", path, json);
    location.write_to_writer(&mut fs::File::create(path).unwrap()).unwrap();
    Ok(api::Response {
        msg: "ok".to_string(),
        noun: None,
    })
}
