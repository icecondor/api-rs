pub mod api;
pub mod config;
pub mod db;
pub mod email;
pub mod net;
pub mod nouns;
pub mod peer;
pub mod pool;
pub mod session;

use once_cell::sync::Lazy;

pub static CONFIG: Lazy<config::Config> = Lazy::new(|| config::load());
