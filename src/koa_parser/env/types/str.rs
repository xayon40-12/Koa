use super::{Any,Type};
use crate::koa_parser::{Res,ScriptIt};
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
                None => {
                    it.c = '\0';
                    break;
                },
                Some(c) => match c {
                    '\\' => s += &'\\'.to_string(), //TODO escape symboles
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
