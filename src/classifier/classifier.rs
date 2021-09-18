use std::{process::Command, str::from_utf8};

pub struct LanguageClassifier;

#[derive(Debug)]
#[non_exhaustive]
pub enum ProgrammingLanguage {
    C,
    Cpp,
    Java,
    Python,
}

impl LanguageClassifier {
    pub fn classify<S: Into<String>>(val: S) -> Option<ProgrammingLanguage> {
        let lang = val.into().to_lowercase();

        if lang.contains("c++") || lang.contains("cpp") {
            return Some(ProgrammingLanguage::Cpp);
        } else if lang.contains("java") {
            return Some(ProgrammingLanguage::Java);
        } else if lang.contains("python") {
            return Some(ProgrammingLanguage::Python);
        } else if lang.contains("c ") {
            return Some(ProgrammingLanguage::C);
        }

        return None
    }

    pub fn classify_ml(val: &str) -> Option<ProgrammingLanguage> {
        let mut output = Command::new("./src/classifier/classifier").args(&[&format!("{}", val)]).output().unwrap();
        let language = from_utf8(&output.stdout).unwrap();

        if "c_cpp" == language {
            return Some(ProgrammingLanguage::Cpp);
        }else if "java" == language {
            return Some (ProgrammingLanguage::Java);
        } else if "python" == language {
            return Some(ProgrammingLanguage::Python);
        }

        None
    }
}