use dgpdb::db as dgp;

pub fn open() -> Db {
    return Db { dgp: dgp::open() };
}

pub struct Db {
    dgp: dgpdb::db::Db,
}

impl Db {
    pub fn file_from_id(&self, id: &String) -> String {
        self.dgp.file_from_id(id)
    }
    pub fn write<T: protobuf::MessageFull>(&self, value: &T) -> String {
        self.dgp.write(value)
    }
}
