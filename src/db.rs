use dgpdb::db as dgp;
use protobuf::Message;
use std::fs;

use crate::nouns::*;

pub fn open() -> Db {
    return Db {
        dgp: dgp::open("schema.json".to_owned()),
    };
}

pub struct Db {
    pub dgp: dgpdb::db::Db,
}

impl Db {
    pub fn filename_from_id(&self, id: &str) -> String {
        self.dgp.filename_from_id(id)
    }
    pub fn write<T: protobuf::MessageFull>(&self, value: &T) -> String {
        self.dgp.put(value)
    }
    pub fn user_by_email(&self, email: &str) -> Result<user::User, dgpdb::Error> {
        let user_id = self.dgp.get("user", "email", email.to_owned())?;
        let filename = self.filename_from_id(&user_id);
        let mut reader = fs::File::open(filename).unwrap();
        let user = user::User::parse_from_reader(&mut reader).unwrap();
        Ok(user)
    }
}
