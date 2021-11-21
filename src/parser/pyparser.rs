use crate::prelude::*;

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
        // let tokens: Vec<_> = PyToken::lexer(&file).collect();
        // let components = &mut self.knowledge_components;
        // let mut token_iter = tokens.iter().enumerate();
        // let time_stamp = format!("{}&t={}", self.source, time_code);

        let tokens: Vec<_> = PyToken::lexer(&file).collect();
        let mut token_iter = tokens.iter().enumerate();
        
        let knowledge_component_set = &mut self.knowledge_components;
        let mut knowledge_component: KnowledgeComponent;
        let mut plain_component: Component;

        let time_stamp = format!("{}&t={}", self.source, time_code);

        while let Some((idx, token)) = token_iter.next() {
            match token {
                PyToken::Float => plain_component = parse_float(&token),
                PyToken::Int => plain_component = parse_integer(&token),
                PyToken::Complex => plain_component = parse_complex(&token),
                PyToken::Bool | PyToken::True | PyToken::False => plain_component = parse_boolean(&token),
                PyToken::List => plain_component = parse_list(&token),
                PyToken::Tuple => plain_component = parse_tuple(&token),
                PyToken::Dict => plain_component = parse_dict(&token),
                PyToken::Set => plain_component = parse_set(&token),
                PyToken::Bytes => plain_component = parse_bytes(&token),
                PyToken::Class => plain_component = parse_class(&token),
                PyToken::String => plain_component = parse_string(&token),
                PyToken::None => plain_component = parse_none(&token),
                PyToken::FunctionDefinition | PyToken::Decorator | 
                PyToken::Final | PyToken::Overload => plain_component = parse_declarator(&token),
                PyToken::Global => plain_component = parse_type_qualifier(&token),
                PyToken::Addition | PyToken::Subtraction | PyToken::Multiplication | PyToken::Division |
                PyToken::FloorDivision | PyToken::Modulo | PyToken::Exponentiation => plain_component = parse_arithmetic(&token),
                PyToken::AddAssignment | PyToken::SubAssignment | PyToken::MultAssignment | PyToken::DivAssignment |
                PyToken::FloorDivAssignment | PyToken::ModAssignment | PyToken::ExpAssignment | PyToken::Assignment | 
                PyToken::AssignmentExpression => plain_component = parse_assignment(&token),
                PyToken::BitwiseAnd | PyToken::BitwiseAndAssignment | PyToken::BitwiseOr | PyToken::BitwiseOrAssignment |
                PyToken::BitwiseXor | PyToken::BitwiseXorAssignment | PyToken::BitwiseNot | PyToken::BitwiseLeftShift |
                PyToken::BitwiseLeftShiftAssignment | PyToken::BitwiseRightShift | PyToken::BitwiseRightShiftAssignment => plain_component = parse_bitwise(&token),
                PyToken::Function => plain_component = parse_function(&token),
                PyToken::LogicalAnd | PyToken::LogicalOr | PyToken::LogicalNot => plain_component = parse_logical(&token),
                PyToken::Greater | PyToken::GreaterOrEquals | PyToken::Less | PyToken::LessOrEquals |
                PyToken::Equal | PyToken::NotEquals  => plain_component = parse_comparison(&token),
                PyToken::MemberAccess => plain_component = parse_member_access(&token),
                PyToken::Is | PyToken::IsNot => plain_component = parse_identity(&token),
                PyToken::In | PyToken::NotIn => plain_component = parse_membership(&token),
                PyToken::Yield | PyToken::Async | PyToken::Await | PyToken::Lambda => plain_component = parse_misc_declaration(&token),
                _ => {
                    continue;
                }
            }
            knowledge_component = KnowledgeComponent::new(plain_component.clone(), token, &time_stamp);
            knowledge_component_set.insert(knowledge_component.clone());
        }
        Ok(())
    }

    fn get_knowledge_components(&self) -> IndexSet<KnowledgeComponent> {
        self.knowledge_components.clone()
    }
}


fn parse_float(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Floating Point Number", Some(token));

    parse_arithmetic_type(component)
}

fn parse_integer(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Integer Number", Some(token));

    parse_arithmetic_type(component)
}

fn parse_complex(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Complex Number", Some(token));

    parse_arithmetic_type(component)
}

fn parse_boolean(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Boolean", Some(token));

    parse_arithmetic_type(component)
}

fn parse_list(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("List", Some(token));

    parse_data_type(component)
}

fn parse_tuple(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Tuple", Some(token));

    parse_data_type(component)
}

fn parse_dict(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Dict", Some(token));

    parse_data_type(component)
}

fn parse_set(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Set", Some(token));

    parse_data_type(component)
}

fn parse_bytes(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Bytes", Some(token));

    parse_data_type(component)
}

fn parse_class(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("ClassType", Some(token));

    parse_data_type(component)
}

fn parse_string(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("StringType", Some(token));

    parse_data_type(component)
}

fn parse_none(token: &PyToken) -> Component {
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

fn parse_declarator(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Declarator", Some(token));

    parse_declaration(component)
}

fn parse_type_qualifier(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Type Qualifier", Some(token));

    parse_declaration(component)
}

fn parse_misc_declaration(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new(format!("{} Declaration", token.name), Some(token));

    parse_declaration(component)
}

fn parse_declaration(token: Component) -> Component {
    let component = Component::new("Declaration", Some(token));
    component
}
// -------------------------------------------------------------------------------------------------
fn parse_arithmetic(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Arithmetic", Some(token));

    parse_expression(component)
}

fn parse_assignment(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Assignment", Some(token));

    parse_expression(component)
}

fn parse_bitwise(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Bitwise", Some(token));

    parse_expression(component)
}

fn parse_logical(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Logical", Some(token));

    parse_expression(component)
}

fn parse_function(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Function", Some(token));

    parse_expression(component)
}

fn parse_comparison(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Comparison", Some(token));

    parse_expression(component)
}

fn parse_member_access(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Member Access", Some(token));

    parse_expression(component)
}

fn parse_identity(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Identity Operator", Some(token));

    parse_expression(component)
}

fn parse_membership(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Membership Operator", Some(token));

    parse_expression(component)
}

fn parse_misc_expression(token: &PyToken) -> Component {
    let token = Component::new(token, None);
    let component = Component::new(format!("{} Expression", token.name), Some(token));

    parse_expression(component)
}

fn parse_expression(token: Component) -> Component {
    let component = Component::new("Expression", Some(token));
    
    component
}