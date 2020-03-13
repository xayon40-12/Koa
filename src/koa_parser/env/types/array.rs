use super::{Any,Type};
use crate::koa_parser::{Res,ParseErr,ScriptIt,KoaParser};
use std::fmt;

pub struct Array {
    array: Vec<Box<dyn Any>>
}

impl Array {
    pub fn parse(mut it: &mut ScriptIt) -> Res {
        match it.chars.next(){
            None => return Err(ParseErr::Parse(String::from("Program ended before Array ended."))),
            Some(c) => it.c = c
        }
        let mut array = Vec::<Box<dyn Any>>::new(); 
        let first = KoaParser::parse_next(&mut it)?;
        let f_type = first.get_type();
        array.push(first);
        loop {
            match it.c {
                ',' => {
                    match it.chars.next(){
                        None => return Err(ParseErr::Parse(String::from("Program ended before Array ended."))),
                        Some(c) => it.c = c
                    }
                }, // continue
                ']' => {
                    it.next();
                    break;
                },
                _ => {
                    let val = KoaParser::parse_next(&mut it)?;
                    if val.get_type() == f_type {
                        array.push(val);
                    } else {
                        return Err(ParseErr::Parse(format!("all array elements must have same type. Expected {:?} found {:?}.", f_type, val.get_type())));
                    }
                }
            }
        }
        Ok(Box::new(Array {array}))
    }
}

impl Any for Array {
    fn get_type(&self) -> Type {
        Type::Array
    }
}


impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "[")?;
        if let Some((last, elements)) = self.array.split_last() {
            for v in elements {
                write!(f, "{}, ", v)?;
            }
            write!(f, "{}", last)?;
        }
        write!(f, "{}", "]")
    }
}
