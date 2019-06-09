pub mod types;
pub use types::Any;
use std::collections::HashMap;  

pub struct Env<'a> {
    parent: Option<&'a Env<'a>>,
    arc: Option<&'a Env<'a>>, // sharded archetype environment 
    content: HashMap<String, Box<dyn Any>>
}

impl<'a> Env<'a> {
    pub fn global() -> Env<'a> {
        let content = HashMap::new();
        Env { parent: None, arc: None, content }
    }
}
