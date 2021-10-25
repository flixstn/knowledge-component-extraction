pub(crate) mod cjparser;
pub(crate) mod pyparser;
pub(crate) mod knowledge_component;

use indexmap::IndexSet;

use crate::{classifier::classifier::ProgrammingLanguage, parser::cjparser::CJParser, parser::pyparser::PyParser};
use std::error::Error;

use self::knowledge_component::KnowledgeComponent;

// TODO: check implementation std::mem::MaybeUninit for parser
#[derive(Debug)]
pub struct ProtoParser {
    pub parser: Option<Box<dyn Parser>>,
}

impl ProtoParser {
    pub fn new() -> Self {
        Self {
            parser: None,
        }
    }
    pub fn parse_language(&mut self, source: &str, language: ProgrammingLanguage) {
        match language {
            ProgrammingLanguage::C => {
                let parser = CJParser::new(source);
                self.parser = Some(Box::new(parser))
            }
            ProgrammingLanguage::Cpp => {
                let parser = CJParser::new(source);
                self.parser = Some(Box::new(parser))
            }
            ProgrammingLanguage::Java => {
                let parser = CJParser::new(source);
                self.parser = Some(Box::new(parser))
            }
            ProgrammingLanguage::Python => {
                let parser = PyParser::new(source);
                self.parser = Some(Box::new(parser))
            }
            _ => {}
        }
    }

    pub fn parse(&mut self, file: &str, time_code: i32) -> Result<(), Box<dyn Error>> {      
        if let Some(res) = self.parser.as_mut() {
            res.parse(file, time_code).unwrap();
        }

        Ok(())
    }

    pub fn get_knowledge_components(&self) -> IndexSet<KnowledgeComponent> {
        self.parser.as_ref().unwrap().get_knowledge_components()
    }
}

pub trait Parser: std::fmt::Debug + Send {
    fn parse(&mut self, file: &str, time_code: i32) -> Result<(), Box<dyn Error>>;
    fn get_knowledge_components(&self) -> IndexSet<KnowledgeComponent>;
}