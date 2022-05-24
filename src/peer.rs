use std::fs;
use std::fs::File;
use std::sync::Arc;

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
            api::Commands::Auth(auth) => auth_op(&self.db, &auth.device_key),
            //_ => Err(format!("not implemented"))
        }
    }
}

pub fn auth_op(db: &db::Db, _device_key: &str) -> PeerResult {
    let user_id = api::Nouns::UserId("abc1".to_string());
    Ok(api::Response {
        msg: "ok".to_string(),
        noun: Some(user_id),
    })
}

pub fn read_op(db: &db::Db, query: &api::QueryById) -> PeerResult {
    let path = db.file_from_id(&query.id);
    match File::open(&path) {
        Ok(reader) => {
            let noun: api::Nouns = serde_json::from_reader(reader).unwrap();
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
    let value = serde_json::to_value(&location).unwrap();
    let id = db.write(&location);
    let path = db.file_from_id(&id);
    let json = serde_json::to_string(&location).unwrap();
    println!("write_op: {} {}", path, json);
    fs::write(path, json).unwrap();
    Ok(api::Response {
        msg: "ok".to_string(),
        noun: None,
    })
}
