use std::fs;
use std::fs::File;
use std::sync::Arc;

use liquid;
use protobuf::Message;
use redis::Commands;

use crate::nouns::*;
use crate::*;

pub struct Peer {
    pub session: Option<api::Session>,
    pub db: Arc<db::Db>,
    pub redis: redis::Connection,
}

pub fn new(db: Arc<db::Db>, redis: redis::Connection) -> Peer {
    Peer {
        session: None,
        db: db,
        redis: redis,
    }
}

impl Peer {
    pub fn command(&mut self, command: api::Commands) -> api::Response {
        match command {
            api::Commands::AuthBySession(device_id) => self.auth_session_op(&device_id),
            api::Commands::AuthByEmail(email) => self.auth_email_op(&email),
            api::Commands::UserDetail(by_username) => self.user_detail_op(by_username),
            api::Commands::UserUpdate(by_username) => self.user_update_op(by_username),
            api::Commands::Read(by_id) => self.read_op(&by_id),
            api::Commands::Write(location) => self.write_op(location),
            _ => api::Response::Error(format!("not implemented")),
        }
    }

    pub fn user_update_op(&mut self, user_update: api::ByUpdatableUser) -> api::Response {
        match &self.session {
            Some(session) => {
                let mut user = self.db.user_by_id(&session.user_id);
                match user_update.username {
                    Some(username) => {
                        println!("updating username {} to {}", user.username, username);
                        user.username = username;
                    }
                    None => (),
                }
                self.db.write(&user);
                api::Response::Result(api::Nouns::None)
            }
            None => api::Response::Error("Login first".to_string()),
        }
    }

    pub fn user_detail_op(&mut self, username: Option<api::ByUsername>) -> api::Response {
        match username {
            Some(by_username) => {
                let user_id = self
                    .db
                    .dgp
                    .get("user", "username", by_username.username)
                    .unwrap();
                let user = self.db.user_by_id(&user_id);
                api::Response::Result(api::Nouns::User(user))
            }
            None => match &self.session {
                Some(session) => {
                    let user = self.db.user_by_id(&session.user_id);
                    api::Response::Result(api::Nouns::User(user))
                }
                None => api::Response::Error("Login or specify a username/id to lookup".to_owned()),
            },
        }
    }

    pub fn auth_session_op(&mut self, device_key: &api::DeviceKey) -> api::Response {
        match self
            .redis
            .hget::<_, _, String>("session_keys", &device_key.device_key)
        {
            Ok(session_json) => {
                println!(
                    "auth.session hget {} => {}",
                    &device_key.device_key, &session_json
                );
                let session: api::Session = serde_json::from_str(&session_json).unwrap();
                let user = self.db.user_by_id(&session.user_id);
                self.session = Some(session);
                api::Response::Result(api::Nouns::User(user))
            }
            Err(_) => api::Response::Error("device_key does not exist".to_owned()),
        }
    }

    //{id:... "method":"auth.email","params":{"email":"a@b.c","device_id":"browser"}}
    //{id:... "result":{"status":"OK"}}
    pub fn auth_email_op(&mut self, email: &api::Email) -> api::Response {
        let user = match self.db.user_by_email(&email.email) {
            Ok(user) => {
                println!("auth_email_op user_by_email {} found", email.email);
                user
            }
            Err(_) => {
                println!("auth_email_op user_by_email {} NOT found", email.email);
                let mut user = user::User::default();
                user.email = email.email.clone();
                user.id = dgpdb::db::id_new(&user);
                self.db.save_to_file(&user);
                self.db.dgp.put(&user);
                user
            }
        };
        let session = session::Session::new(email.device_id.to_owned(), user.id);
        let json = serde_json::to_string(&session).unwrap();
        println!("hset {} {}", session.id, json);

        match self
            .redis
            .hset::<_, _, String, String>("session_keys", &session.id, json)
        {
            Ok(field_count) => {
                println!("auth_email_op hset {}", field_count)
            }
            Err(_) => {}
        }

        let template = email::signin();
        let globals = liquid::object!({
            "session_key": session.id
        });
        let html = template.render(&globals).unwrap();
        println!("HTML {}", html);

        let user_id = api::Nouns::Id(api::ById {
            id: session.id,
        });
        api::Response::Result(user_id)
    }

    pub fn read_op(&mut self, query: &api::ById) -> api::Response {
        let path = self.db.filename_from_id(&query.id);
        match File::open(&path) {
            Ok(mut reader) => {
                let location = nouns::location::Location::parse_from_reader(&mut reader).unwrap();
                let noun: api::Nouns = api::Nouns::Location(location);
                api::Response::Result(noun)
            }
            Err(e) => {
                println!("read_op: {} {}", path, e);
                api::Response::Error(e.to_string())
            }
        }
    }

    pub fn write_op(&mut self, location: nouns::location::Location) -> api::Response {
        let id = self.db.dgp.put(&location);
        let path = self.db.filename_from_id(&id);
        println!("write_op: {}", path);
        location
            .write_to_writer(&mut fs::File::create(path).unwrap())
            .unwrap();
        api::Response::Result(api::Nouns::Id(api::ById { id: id }))
    }
}
