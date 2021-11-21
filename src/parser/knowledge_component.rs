use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd)]
pub struct KnowledgeComponent {
    token: String,
    value: String,
    #[serde(rename = "timeStamp")]
    time_stamp: String,
    classification: Component,
}

impl KnowledgeComponent {
    pub fn new<T: Into<String> + Copy>(classification: Component, value: T, time_stamp: &str) -> Self {
        Self {
            token: value.into(),
            value: value.into().to_lowercase(),
            time_stamp: time_stamp.into(),
            classification: classification,
        }
    }

    pub fn new_with_ident(classification: Component, value: &str, ident: &str, time_stamp: &str) -> Self {
        Self {
            token: value.into(),
            value: ident.into(),
            time_stamp: time_stamp.into(),
            classification: classification,
        }
    }
}

impl PartialEq for KnowledgeComponent {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

impl Eq for KnowledgeComponent {}

impl Hash for KnowledgeComponent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token.hash(state);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub struct Component {
    pub name: String,
    pub node: Box<Option<Component>>
}

impl Component {
    pub fn new<S: Into<String>>(name: S, children: Option<Component>) -> Self {
        Self {
            name: name.into(),
            node: Box::new(children),
        }
    }
}