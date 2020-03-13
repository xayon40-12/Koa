use super::{Any,Type};
use crate::koa_parser::{Res,ParseErr,ScriptIt,KoaParser};
use std::fmt;

pub struct Tuple {
    array: Vec<Box<dyn Any>>
}

impl Tuple {
    pub fn parse(mut it: &mut ScriptIt, first: Box<dyn Any>) -> Res {
        let mut array = Vec::<Box<dyn Any>>::new(); 
        array.push(first);
        loop {
            match &it.c {
                ',' => {
                    match it.chars.next(){
                        None => return Err(ParseErr::Parse(String::from("Program ended before Array ended."))),
                        Some(c) => it.c = c
                    }
                }, // continue
                ')' => {
                    it.next();
                    break;
                },
                _ => array.push(KoaParser::parse_next(&mut it)?)
            }
        }
        Ok(Box::new(Tuple {array}))
    }
}

impl Any for Tuple {
    fn get_type(&self) -> Type {
        Type::Tuple
    }
}


impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "(")?;
        if let Some((last, elements)) = self.array.split_last() {
            for v in elements {
                write!(f, "{}, ", v)?;
            }
            write!(f, "{}", last)?;
        }
        write!(f, "{}", ")")
    }
}
