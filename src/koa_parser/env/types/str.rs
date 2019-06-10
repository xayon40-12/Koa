use super::{Any,Type};
use crate::koa_parser::{Res,ParseErr,ScriptIt};
use std::fmt;

pub struct Str {
    msg: String
}

impl Str {
    pub fn parse(mut it: &mut ScriptIt) -> Res {
        let mut s = String::new();
        loop {
            let c = it.chars.next();
            match c {
                None => return Err(ParseErr::Parse(String::from("Program ended before String ended."))),
                Some(c) => match c {
                    '\\' => {
                        match it.chars.next(){
                            None => return Err(ParseErr::Parse(String::from("Program ended before string ended."))),
                            Some(c) => s += &Self::escape(c)
                        }
                    },
                    '"' => {
                        match it.chars.next(){
                            None => it.c = '\0',
                            Some(c) => it.c = c
                        }
                        break;
                    },
                    _ => s += &c.to_string()
                }
            }
        }
        Ok(Box::new(Str {msg: s}))
    }
    fn escape(c: char) -> String {
        match c {
            '"' | '\\' => c.to_string(),
            'n' => '\n'.to_string(),
            't' => '\t'.to_string(),
            'r' => '\r'.to_string(),
            _ => format!("\\{}", c)
        }
    }
}

impl Any for Str {
    fn get_type(&self) -> Type {
        Type::Str
    }
}


impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}
