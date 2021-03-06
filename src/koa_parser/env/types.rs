use std::fmt;
pub mod str; pub use self::str::Str;
pub mod num; pub use num::Num;  
pub mod array; pub use array::Array;
pub mod tuple; pub use tuple::Tuple;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Type {
    None,
    Nev,
    Var,
    Num,
    Str,
    Array,
    Func,
    Tuple,
    Table,
    Obj
}

pub trait Any: fmt::Display {
    fn get_type(&self) -> Type;
}
