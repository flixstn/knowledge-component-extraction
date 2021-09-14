pub(crate) mod cjparser;
pub(crate) mod pyparser;
pub(crate) mod knowledge_component;

use std::error::Error;

pub trait Parser {
    fn parse(&mut self, file: &str, time_code: i32) -> Result<(), Box<dyn Error>>;
}