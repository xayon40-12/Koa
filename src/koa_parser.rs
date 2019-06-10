use std::str::Chars;
pub mod env;
pub use env::Env;
pub use env::types::{Any,Str,Num,Array};

pub struct KoaParser<'a> {
    env: Env<'a>
}

pub type Res = Result<Box<dyn Any>,ParseErr>;

pub enum ParseErr {
    Parse(String),
    Eof
}

pub struct ScriptIt<'a,'b> {
    env: &'a mut Env<'b>,
    c: char,
    chars: &'a mut Chars<'a>,
    line_id: u32
}

impl<'a> KoaParser<'a> {
    pub fn new() -> KoaParser<'a>{
        KoaParser{env: Env::global()}
    }
    pub fn parse(&mut self, prog: String) {
        let chars = &mut prog.chars();
        let mut it = ScriptIt {env: &mut self.env, c: chars.next().expect("Parse Error: Empty program. Abort."), chars, line_id: 1};
        loop {
            let v = Self::parse_next(&mut it);
            match v {
                Ok(val) => {
                    //TODO
                    println!("l:{}: ({:?}) {}", it.line_id, val.get_type(), &val);
                },
                Err(e) => match e {
                    ParseErr::Parse(msg) => panic!("Parsing Error l:{}: {}", it.line_id, msg),
                    ParseErr::Eof => break
                }
            }
        }
    }
    fn parse_next(mut it: &mut ScriptIt) -> Res {
        Self::skip_white_space(&mut it);
        let res = match it.c {
            '0' ..= '9' => Num::parse(&mut it),
            '"' => Str::parse(&mut it),
            '{' => Array::parse(&mut it),
            '\n' => {
                it.line_id += 1;
                Self::parse_next_char(&mut it);
                Self::parse_next(&mut it)
            },
            '\0' => Err(ParseErr::Eof),
            _ => Err(ParseErr::Parse(format!("Unrecognised character '{}'.", &it.c)))
        };
        Self::skip_white_space(&mut it);
        res
    }
    fn skip_white_space(mut it: &mut ScriptIt) {
        while it.c == ' ' {
            Self::parse_next_char(&mut it);
        }
    }
    fn parse_next_char(mut it: &mut ScriptIt) {
        match it.chars.next() {
            Some(c) => it.c = c,
            None => it.c = '\0'
        }
    }
}
