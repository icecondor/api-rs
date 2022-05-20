use mile39;
use serde::{Deserialize, Serialize};

pub mod command;
pub mod location;

#[derive(Serialize, Deserialize)]
pub enum Nouns {
    Location(mile39::nouns::location::Location),
    UserId(String),
}
