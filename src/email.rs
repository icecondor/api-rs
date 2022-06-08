use liquid;
use std::fs;
use std::path;

use crate::CONFIG;

pub fn load_template(name: &str) -> String {
    let filename = path::Path::new(&CONFIG.template_path).join(&name);
    fs::read_to_string(filename).unwrap()
}

pub fn signin() -> liquid::Template {
    liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(&load_template("signin"))
        .unwrap()
}
