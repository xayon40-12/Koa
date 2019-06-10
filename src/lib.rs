pub mod koa_parser;
pub use koa_parser::KoaParser;

#[cfg(test)]
mod test {
    use super::KoaParser;

    #[test]
    fn print_var() {
        let script = String::from(r#"
123.4 {313, "natoh\\eu09348u",{0988,0,"aoecu7a9e","\""}} 4094
24 2049 "aon\nau89&$    %oe7\tuhdu tnoehu lrecu" 43
098.09
"#);
        let mut parser = KoaParser::new();
        parser.parse(script);
    }
}
