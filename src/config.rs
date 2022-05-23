use std::fs;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub addr: String
}

pub fn load() -> Config {
  let reader = fs::File::open("config.json").unwrap();
  serde_json::from_reader(reader).unwrap()
}
