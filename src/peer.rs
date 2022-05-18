use std::fs;
use std::fs::File;
use std::sync::Arc;

use serde_json;

use crate::nouns;
use crate::nouns::*;

use mile39::db;

pub struct Peer {
    pub user_id: Option<String>,
    pub db: Arc<db::Db>,
}

type PeerResult = Result<command::Response, String>;

pub fn new(db: Arc<db::Db>) -> Peer {
    Peer {
        user_id: None,
        db: db,
    }
}

impl Peer {
    pub fn command(&self, line: &str) -> PeerResult {
        let command: command::Commands = serde_json::from_str(&line).unwrap();
        println!("{}", serde_json::to_string(&command).unwrap());
        self.do_command(command)
    }

    pub fn do_command(&self, command: command::Commands) -> PeerResult {
        match command {
            command::Commands::Read(read) => read_op(&self.db, &read.params),
            command::Commands::Write(write) => write_op(&self.db, write.params),
            command::Commands::Auth(auth) => auth_op(&self.db, &auth.device_key),
            //_ => Err(format!("not implemented"))
        }
    }
}

pub fn auth_op(db: &mile39::db::Db, device_key: &str) -> PeerResult {
    let user_id = nouns::Nouns::UserId("abc1".to_string());
    Ok(command::Response {
        msg: "ok".to_string(),
        noun: Some(user_id)
    })
}

pub fn read_op(db: &mile39::db::Db, query: &command::QueryById) -> PeerResult {
    let path = db.file_from_id(&query.id);
    match File::open(&path) {
        Ok(reader) => {
            let noun: nouns::Nouns = serde_json::from_reader(reader).unwrap();
            Ok(command::Response {
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

pub fn write_op(db: &mile39::db::Db, location: mile39::nouns::location::Location) -> PeerResult {
    let wrapped_location = &mile39::nouns::Nouns::Location(location);
    let id = db.write(&wrapped_location);
    let path = db.file_from_id(&id);
    fs::write(path, serde_json::to_string(&wrapped_location).unwrap()).unwrap();
    Ok(command::Response {
        msg: "ok".to_string(),
        noun: None,
    })
}
