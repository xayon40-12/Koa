use super::{Any,Type};
use crate::koa_parser::{Res,ParseErr,ScriptIt,KoaParser};
use std::fmt;

pub struct Array {
    array: Vec<Box<dyn Any>>
}

impl Array {
    pub fn parse(mut it: &mut ScriptIt) -> Res {
        let mut array = Vec::<Box<dyn Any>>::new(); 
        match it.chars.next(){
            None => return Err(ParseErr::Parse(String::from("Program ended before Array ended."))),
            Some(c) => it.c = c
        }
        loop {
            match &it.c {
                ',' => {
                    match it.chars.next(){
                        None => return Err(ParseErr::Parse(String::from("Program ended before Array ended."))),
                        Some(c) => it.c = c
                    }
                }, // continue
                '}' => {
                    match it.chars.next(){
                        None => it.c = '\0',
                        Some(c) => it.c = c
                    }
                    break;
                },
                _ => array.push(KoaParser::parse_next(&mut it)?)
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
        write!(f, "{}", "{")?;
        if let Some((last, elements)) = self.array.split_last() {
            for v in elements {
                write!(f, "{}, ", v)?;
            }
            write!(f, "{}", last)?;
        }
        write!(f, "{}", "}")
    }
}
