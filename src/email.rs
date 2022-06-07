use liquid;

pub fn signin() -> liquid::Template {
        liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse("{{session_key}}")
            .unwrap()
}
