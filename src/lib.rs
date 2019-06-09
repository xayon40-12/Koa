pub mod koa_parser;
pub use koa_parser::KoaParser;

#[cfg(test)]
mod test {
    use super::KoaParser;

    #[test]
    fn print_var() {
        let script = String::from("123.4 4094\n 24 2049 \"aonau89&$\t%oe7\\tuhdu tnoehu lrecu\" 43 \n   098.09");
        let mut parser = KoaParser::new(script);
        parser.parse();
    }
}
