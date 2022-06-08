use liquid;

use crate::CONFIG;

pub struct Email {
    template_path: String,
}

pub fn init(path: String) -> Email {
    return Email {
        template_path: path,
    };
}
pub fn signin() -> liquid::Template {
    let _f = CONFIG.addr.to_owned();
    liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse("{{session_key}}")
        .unwrap()
}
