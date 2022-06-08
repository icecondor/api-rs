use liquid;
use std::fs;
use std::path;

use crate::CONFIG;

pub fn load_template(name: String) {
    let filename = path::Path::new(&CONFIG.template_path).join(&name);
    let reader = fs::File::open(filename).unwrap();
}

pub fn signin() -> liquid::Template {
    liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse("{{session_key}}")
        .unwrap()
}
