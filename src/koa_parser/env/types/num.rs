use super::{Any,Type};
use crate::koa_parser::{Res,ParseErr,ScriptIt};
use std::fmt;

pub struct Num {
    val: f64
}

impl Num {
    pub fn parse(mut it: &mut ScriptIt) -> Res {
        let mut num = it.c.to_string();
        loop {
            let c = it.chars.next();
            match c {
                None => {
                    it.c = '\0';
                    break;
                },
                Some(c) => match c {
                    '0' ..= '9' => num += &c.to_string(),
                    '.' => num += &'.'.to_string(),
                    _ => {
                        it.c = c;
                        break;
                    }
                }
            }
        }
        return match num.parse::<f64>() {
            Ok(val) => Ok(Box::new(Num {val})),
            Err(e) => Err(ParseErr::Parse(String::from(e.to_string())))
        }
    }
}

impl Any for Num {
    fn get_type(&self) -> Type {
        Type::Num
    }
}


impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.val)
    }
}
