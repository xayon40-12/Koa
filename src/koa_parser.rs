use std::str::Chars;
pub mod env;
pub use env::Env;
pub use env::types::{Any,Str,Num,Array,Tuple};

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
        it.skip_white_space();
        let res = match it.c {
            '0' ..= '9' => Num::parse(&mut it),
            '"' => Str::parse(&mut it),
            '[' => Array::parse(&mut it),
            '(' => Self::parse_bracket(&mut it),
            '\n' => {
                it.line_id += 1;
                it.next();
                Self::parse_next(&mut it)
            },
            '\0' => Err(ParseErr::Eof),
            _ => Err(ParseErr::Parse(format!("Unrecognised character '{}'.", &it.c)))
        };
        it.skip_white_space();
        res
    }
    fn parse_bracket(mut it: &mut ScriptIt) -> Res {
        it.next();
        if it.c == '\0' {
            return Err(ParseErr::Parse(String::from("Program ended before bracket closed.")))
        }
        let first = Self::parse_next(&mut it)?;
        match it.c {
            ',' => Tuple::parse(&mut it, first),
            _ => Err(ParseErr::Parse(String::from("Bracket not implemented yet.")))
        }
    }
}

impl<'a,'b> ScriptIt<'a,'b> {
    fn next(&mut self) {
        match self.chars.next() {
            Some(c) => self.c = c,
            None => self.c = '\0'
        }
    }
    fn skip_white_space(&mut self) {
        while self.c == ' ' {
            self.next();
        }
    }
}
