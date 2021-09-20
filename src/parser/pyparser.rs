use crate::{
    lexer::pylexer::Token,
    parser::Parser,
    parser::knowledge_component::{KnowledgeComponent, Component},
};
use indexmap::IndexSet;
use logos::Logos;
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct PyParser {
    pub source: String,
    pub knowledge_components: IndexSet<KnowledgeComponent>,
}

impl PyParser {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.into(),
            knowledge_components: IndexSet::new(),
        }
    }
}

impl Parser for PyParser { 
    fn parse(&mut self, file: &str, time_code: i32) -> Result<(), Box<dyn Error>> {
        let tokens: Vec<_> = Token::lexer(&file).collect();
        let components = &mut self.knowledge_components;
        let mut token_iter = tokens.iter().enumerate();
        let time_stamp = format!("{}&t={}", self.source, time_code);

        while let Some((idx, token)) = token_iter.next() {
            match token {
                Token::Float => {
                    let classification = parse_float(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Int => {
                    let classification = parse_integer(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Complex => {
                    let classification = parse_complex(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::True => {
                    let classification = parse_boolean(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::False => {
                    let classification = parse_boolean(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Bool => {
                    let classification = parse_boolean(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::List => {
                    let classification = parse_list(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Tuple => {
                    let classification = parse_tuple(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Dict => {
                    let classification = parse_dict(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Set => {
                    let classification = parse_set(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Bytes => {
                    let classification = parse_bytes(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Class => {
                    let classification = parse_class(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::String => {
                    let classification = parse_string(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::None => {
                    let classification = parse_none(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::FunctionDefinition => {
                    let classification = parse_declarator(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Decorator => {
                    let classification = parse_declarator(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Final => {
                    let classification = parse_declarator(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Overload => {
                    let classification = parse_declarator(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Global => {
                    let classification = parse_type_qualifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Addition => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::AddAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Subtraction => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::SubAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Multiplication => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::MultAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Division => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::DivAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::FloorDivision => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::FloorDivAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Modulo => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::ModAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Exponentiation => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::ExpAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Assignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::AssignmentExpression => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseAnd => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseAndAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseOr => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseOrAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseXor => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseXorAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseNot => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseLeftShift => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseLeftShiftAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseRightShift => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseRightShiftAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::LogicalAnd => {
                    let classification = parse_logical(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::LogicalOr => {
                    let classification = parse_logical(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::LogicalNot => {
                    let classification = parse_logical(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Function => {
                    let classification = parse_function(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Greater => {
                    let classification = parse_comparison(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::GreaterOrEquals => {
                    let classification = parse_comparison(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Less => {
                    let classification = parse_comparison(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::LessOrEquals => {
                    let classification = parse_comparison(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Equal => {
                    let classification = parse_comparison(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::NotEquals => {
                    let classification = parse_comparison(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::MemberAccess => {
                    let classification = parse_member_access(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Is => {
                    let classification = parse_identity(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::IsNot => {
                    let classification = parse_identity(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::In => {
                    let classification = parse_membership(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::NotIn => {
                    let classification = parse_membership(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Yield => {
                    let classification = parse_misc_expression(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Async => {
                    let classification = parse_misc_declaration(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Await => {
                    let classification = parse_misc_expression(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Lambda => {
                    let classification = parse_misc_expression(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Comment => {}
                Token::Error => {}
            }
        }
        Ok(())
    }

    fn get_knowledge_components(&self) -> IndexSet<KnowledgeComponent> {
        self.knowledge_components.clone()
    }
}


fn parse_float(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Floating Point Number", Some(token));

    parse_arithmetic_type(component)
}

fn parse_integer(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Integer Number", Some(token));

    parse_arithmetic_type(component)
}

fn parse_complex(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Complex Number", Some(token));

    parse_arithmetic_type(component)
}

fn parse_boolean(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Boolean", Some(token));

    parse_arithmetic_type(component)
}

fn parse_list(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("List", Some(token));

    parse_data_type(component)
}

fn parse_tuple(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Tuple", Some(token));

    parse_data_type(component)
}

fn parse_dict(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Dict", Some(token));

    parse_data_type(component)
}

fn parse_set(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Set", Some(token));

    parse_data_type(component)
}

fn parse_bytes(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Bytes", Some(token));

    parse_data_type(component)
}

fn parse_class(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("ClassType", Some(token));

    parse_data_type(component)
}

fn parse_string(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("StringType", Some(token));

    parse_data_type(component)
}

fn parse_none(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("NoneType", Some(token));

    parse_data_type(component)
}

fn parse_arithmetic_type(token: Component) -> Component {
    let component = Component::new("Arithmetic Data Type", Some(token));
    
    parse_data_type(component)
}

fn parse_data_type(token: Component) -> Component {
    let component = Component::new("Data Type", Some(token));

    parse_declaration(component)
}

fn parse_declarator(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Declarator", Some(token));

    parse_declaration(component)
}

fn parse_type_qualifier(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Type Qualifier", Some(token));

    parse_declaration(component)
}

fn parse_misc_declaration(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new(format!("{} Declaration", token.name), Some(token));

    parse_declaration(component)
}

fn parse_declaration(token: Component) -> Component {
    let component = Component::new("Declaration", Some(token));
    component
}
// -------------------------------------------------------------------------------------------------
fn parse_arithmetic(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Arithmetic", Some(token));

    parse_expression(component)
}

fn parse_assignment(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Assignment", Some(token));

    parse_expression(component)
}

fn parse_bitwise(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Bitwise", Some(token));

    parse_expression(component)
}

fn parse_logical(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Logical", Some(token));

    parse_expression(component)
}

fn parse_function(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Function", Some(token));

    parse_expression(component)
}

fn parse_comparison(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Comparison", Some(token));

    parse_expression(component)
}

fn parse_member_access(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Member Access", Some(token));

    parse_expression(component)
}

fn parse_identity(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Identity Operator", Some(token));

    parse_expression(component)
}

fn parse_membership(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Membership Operator", Some(token));

    parse_expression(component)
}

fn parse_misc_expression(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new(format!("{} Expression", token.name), Some(token));

    parse_expression(component)
}

fn parse_expression(token: Component) -> Component {
    let component = Component::new("Expression", Some(token));
    
    component
}