use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CJParser {
    pub source: String,
    pub knowledge_components: IndexSet<KnowledgeComponent>,
}

impl CJParser {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.into(),
            knowledge_components: IndexSet::new()
        }
    }    
}

impl Parser for CJParser {
    fn parse(&mut self, file: &str, time_code: i32) -> Result<(), Box<dyn Error>> {
        let tokens: Vec<_> = Token::lexer(&file).collect();
        let mut token_iter = tokens.iter().enumerate();
        
        let knowledge_component_set = &mut self.knowledge_components;
        let mut knowledge_component: KnowledgeComponent;
        let mut plain_component: Component;

        let time_stamp = format!("{}&t={}", self.source, time_code);

        // eprintln!("{:?}", tokens.clone());
        while let Some((idx, token)) = token_iter.next() {
            match token {
                // Preprocessor Classification
                // preprocessor
                Token::Preprocessor(ident) => {
                    plain_component = parse_preprocessor(&token);
                }
                // Statement Classification
                // iteration
                Token::For | Token::While | Token::DoWhile => {
                    plain_component = parse_iteration(&token);
                }
                // selection
                Token::Switch | Token::Case | Token::Default | Token::If | Token::ElseIf | Token::Else => {
                    plain_component = parse_condition(&token);
                }
                // jump
                Token::Break | Token::Continue | Token::Goto | Token::Return => {
                    plain_component = parse_jump(&token);
                }
                // Declaration Classification
                // arithmetic type
                Token::Unsigned | Token::Signed => {
                    plain_component = parse_sign(&token);
                }
                Token::Char => {
                    plain_component = parse_character(&token);
                }
                Token::Int | Token::Short | Token::ShortInt | Token::Long | Token::LongInt | Token::LongLong | Token::LongLongInt => {
                    plain_component = parse_integer(&token);
                }
                Token::Float | Token::Double | Token::LongDouble => {
                    plain_component = parse_float(&token);
                }
                Token::Bool => {
                    plain_component = parse_boolean(&token);
                }
                Token::String => {
                    plain_component = parse_string(&token);
                }
                // elaborated type specifier
                Token::Enum | Token::Struct | Token::Union | Token::Class | Token::Template => {
                    // TODO: Template: Data Type -> Abstract Data Type -> Data Type -> Statement
                    plain_component = parse_elaborated_type_specifier(&token);
                }
                Token::Void => {
                    plain_component = parse_void(&token);
                }
                // declarators
                // array
                Token::OpenBracket => {
                    // TODO: refine implementation
                    plain_component = parse_declarator(&Token::Array);
                    knowledge_component = KnowledgeComponent::new(plain_component, &Token::Array, &time_stamp);
                    knowledge_component_set.insert(knowledge_component);
                    continue;
                }
                // function
                Token::OpenParen => {
                    // TODO: refine implementation
                    plain_component = parse_declarator(&Token::Function);
                    knowledge_component = KnowledgeComponent::new(plain_component, &Token::Function, &time_stamp);
                    knowledge_component_set.insert(knowledge_component);
                    continue;
                }
                // IDENTIFIER       // VARIABLE
                // pointer
                Token::Asterisk => {
                    let peek = tokens.get(idx-1).unwrap_or(&Token::Error);
                    match &*peek {
                        // check for multiplication
                        Token::Identifier(_) | Token::Number(_) => {
                            plain_component = parse_arithmetic(&Token::Multiplication);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::Multiplication, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                        // check for pointer operation
                        _ => {
                            plain_component = parse_declarator(&Token::Pointer);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::Pointer, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                    }
                }
                // TYPEDEFNAME

                // storage class
                Token::Auto | Token::Extern | Token::Register | Token::Static | Token::ThreadLocal | Token::Typedef | Token::Decltype => {
                    plain_component = parse_storage_class(&token);
                }
                // type qualifier
                Token::Atomic | Token::Const | Token::Restrict | Token::Volatile | Token::Mutable | Token::Public | Token::Protected | Token::Private => {
                    plain_component = parse_access_specifier(&token);
                }
                // Expression Declaration
                // arithmetic
                Token::Plus | Token::Minus | Token::Multiplication | Token::Divide | Token::Modulo => {
                    plain_component = parse_arithmetic(&token);
                }
                // assignment
                Token::AddAssignment | Token::SubAssignment | Token::MultAssignment | Token::DivAssignment | Token::ModAssignment | Token::Assignment => {
                    plain_component = parse_assignment(&token);
                }
                // bitwise
                Token::Ampersand | Token::BitwiseAnd | Token::Reference => {
                    // TODO: potential 'attempt to subtract with overflow'
                    let peek = tokens.get(idx-1).unwrap_or(&Token::Error);
                    match &*peek {
                        Token::Identifier(_ident) | Token::Number(_ident) => {
                            plain_component = parse_bitwise(&Token::BitwiseAnd);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::BitwiseAnd, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                        _ => {
                            plain_component = parse_bitwise(&Token::Reference);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::Reference, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                    }
                }
                Token::BitwiseOr | Token::BitwiseXor | Token::BitwiseNot | Token::LeftShiftAssignment | Token::RightShiftAssignment | Token::BitwiseAndAssignment | 
                Token::BitwiseOrAssignment | Token::BitwiseXorAssignment | Token::UnsignedRightShiftOperator => {
                    plain_component = parse_bitwise(&token);
                }
                Token::LeftOperator => {
                    let peek = tokens.get(idx-1).unwrap_or(&Token::Error);
                    match &*peek {
                        // check whether lhs is stream operator
                        Token::Identifier(ident) => {
                            match ident.as_str() {
                                // TODO: skip tokens until linebreak
                                "cout" | "cerr" | "clog" => {
                                    continue;
                                }
                                _ => {
                                    plain_component = parse_bitwise(&Token::LeftShift);
                                    knowledge_component = KnowledgeComponent::new(plain_component, &Token::LeftShift, &time_stamp);
                                    knowledge_component_set.insert(knowledge_component);
                                    continue;
                                }
                            }
                        }
                        _ => {
                            plain_component = parse_bitwise(&Token::LeftShift);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::LeftShift, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                    }
                    plain_component = parse_bitwise(&token);
                }
                Token::RightOperator => {
                    let peek = tokens.get(idx-1).unwrap_or(&Token::Error);
                    match &*peek {
                        // check whether lhs is stream operator
                        Token::Identifier(ident) => {
                            match ident.as_str() {
                                // TODO: skip tokens until linebreak
                                "cin" => {
                                    continue;
                                }
                                _ => {
                                    plain_component = parse_bitwise(&Token::RightShift);
                                    knowledge_component = KnowledgeComponent::new(plain_component, &Token::RightShift, &time_stamp);
                                    knowledge_component_set.insert(knowledge_component);
                                    continue;
                                }
                            }
                        }
                        _ => {
                            plain_component = parse_bitwise(&Token::RightShift);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::RightShift, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                    }
                }
                // logical
                Token::And | Token::Or | Token::Not => {
                    plain_component = parse_logical(&token);
                }
                // sizeof
                Token::SizeOf => {
                    plain_component = parse_size_of(&token);
                }
                // initialization
                // DESIGNATED INITIALIZER
                // EQUALS INITIALIZER
                // INITIALIZATION LIST
                
                // function call
                // FUNCTION CALL
                
                // typecast
                Token::TypeCast => {
                    plain_component = parse_type_cast(&token);
                }
                // conditional operator
                Token::ConditionalOperator => {
                    plain_component = parse_conditional_operator(&token);
                }
                // COMMA OPERATOR
                // EXPRESSION LIST

                // comparison
                Token::GreaterOrEquals | Token::LessOrEquals | Token::Equals | Token::NotEquals | Token::ThreeWayComparison => {
                    plain_component = parse_comparison(&token);
                }
                Token::Less | Token::Greater => {
                    // TODO: refine implementation
                    let peek_forward = tokens.get(idx+1).unwrap_or(&Token::Error);
                    let peek_backword = tokens.get(idx-1).unwrap_or(&Token::Error);

                    match (peek_forward, peek_backword) {
                        (Token::Number(_), Token::Identifier(_)) => {
                            plain_component = parse_comparison(&token);
                        }
                        _ => continue
                    }

                    plain_component = parse_comparison(&token);
                }
                // member access
                Token::DotOperator | Token::ArrowOperator=> {
                    // TODO: implement 
                    // plain_component = parse_member_access(&token);
                    continue;
                }
                // increment decrement
                Token::Increment => {
                    let peek = tokens.get(idx+1).unwrap_or(&Token::Error);
                    match &*peek {
                        // check whether rhs is variable | number
                        Token::Identifier(_ident) | Token::Number(_ident) => {
                            plain_component = parse_increment(&Token::PrefixIncrement);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::PrefixIncrement, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                        _ => {
                            // check whether lhs is variable | number
                            plain_component = parse_increment(&Token::PostfixIncrement);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::PostfixIncrement, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                    }
                }
                Token::Decrement => {
                    let peek = tokens.get(idx+1).unwrap_or(&Token::Error);
                    match &*peek {
                        // check whether rhs is variable | number
                        Token::Identifier(_ident) | Token::Number(_ident) => {
                            plain_component = parse_decrement(&Token::PrefixDecrement);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::PrefixDecrement, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                        // check whether lhs is variable | number
                        _ => {
                            plain_component = parse_decrement(&Token::PostfixDecrement);
                            knowledge_component = KnowledgeComponent::new(plain_component, &Token::PostfixDecrement, &time_stamp);
                            knowledge_component_set.insert(knowledge_component);
                            continue;
                        }
                    }    
                }
                // memory allocation
                Token::New | Token::Delete => {
                    plain_component = parse_allocation(&token);
                }
                // scope resolution
                Token::ScopeResolution => {
                    plain_component = parse_scope_resolution(&token);
                }
                Token::True | Token::False | Token::Null => {
                    plain_component = parse_predefined_constant(&token);
                }
                Token::Using => {
                    plain_component = parse_using(&token);
                }
                Token::Namespace => {
                    plain_component = parse_namespace(&token);
                }
                // --------------------------------------------------------
                // compilation unit
                Token::Import | Token::Package => {
                    plain_component = parse_compilation_unit(&token);
                }
                // modifier
                Token::Annotation | Token::Abstract | Token::Final | Token::Native | Token::Synchronized | Token::Transient | Token::StrictFp | Token::Assert => {
                    plain_component = parse_modifier(&token);
                }
                Token::Interface => {
                    plain_component = parse_abstract_data_type(&token);
                }
                // try/catch
                Token::Try | Token::Catch | Token::Finally=> {
                    plain_component = parse_try_block(&token);
                }
                Token::Throw | Token::Throws => {
                    plain_component = parse_throw(&token);
                }
                // primary expression
                Token::Super => {
                    plain_component = parse_super(&token);
                }
                Token::This => {
                    plain_component = parse_this(&token);
                }
                Token::Extends | Token::Implements => {
                    plain_component = parse_class_extension(&token);
                }
                Token::Var => {
                    plain_component = parse_var(&token);
                }
                _ => {
                    // plain_component = parse_iteration(&token);
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

// Preprocessor -----------------------------------------------------
fn parse_preprocessor(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Preprocessor", Some(token));

    component
}
// ------------------------------------------------------------------

// Statement --------------------------------------------------------
fn parse_iteration(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Iteration", Some(token));

    parse_statement(component)
}
// ------------------------------------------------------------------

fn parse_jump(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Jump", Some(token));

    parse_statement(component)
}

fn parse_label(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Label", Some(token));

    parse_statement(component)
}

fn parse_condition(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Condition", Some(token));

    parse_statement(component)
}

fn parse_statement(token: Component) -> Component {
    let component = Component::new("Statement", Some(token));
    
    component
}
// ------------------------------------------------------------------

// Declaration ------------------------------------------------------
fn parse_sign(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Sign", Some(token));
    
    parse_arithmetic_type(component)
}

fn parse_character(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Character", Some(token));

    parse_arithmetic_type(component)
}

fn parse_integer(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Integer Number", Some(token));

    parse_arithmetic_type(component)
}

fn parse_float(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Floating Point Number", Some(token));

    parse_arithmetic_type(component)
}

fn parse_arithmetic_type(token: Component) -> Component {
    let component = Component::new("Arithmetic Data Type", Some(token));
    
    parse_data_type(component)
}

fn parse_boolean(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Boolean", Some(token));
    
    parse_data_type(component)
}

fn parse_string(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("String", Some(token));
    
    parse_data_type(component)
}

fn parse_elaborated_type_specifier(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Elaborated Type Specifier", Some(token));

    parse_data_type(component)
}

fn parse_void(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Void", Some(token));

    parse_data_type(component)
}

fn parse_data_type(token: Component) -> Component {
    let component = Component::new("Data Type", Some(token));

    parse_declaration(component)
}

fn parse_using(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Using", Some(token));

    parse_declaration(component)
}

fn parse_namespace(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Namespace", Some(token));

    parse_declaration(component)
}

fn parse_declarator(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Declarator", Some(token));

    parse_declaration(component)
}

// TODO: check "Storage Class" whether it is correct
fn parse_storage_class(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Storage Class", Some(token));

    parse_declaration(component)
}

// TODO: check "Storage Class" whether it is correct
fn parse_type_qualifier(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Storage Class", Some(token));

    parse_declaration(component)
}

fn parse_access_specifier(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Access Specifier", Some(token));

    parse_declaration(component)
}

fn parse_declaration(token: Component) -> Component {
    let component = Component::new("Declaration", Some(token));
    component
}
// ------------------------------------------------------------------
// Expression -------------------------------------------------------
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

fn parse_size_of(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Size Of", Some(token));

    parse_expression(component)
}

fn parse_initialization(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Initialization", Some(token));

    parse_expression(component)
}

fn parse_function_call(token: &Token) -> Component {
    let token = Component::new(token, None);

    parse_expression(token)
}

fn parse_type_cast(token: &Token) -> Component {
    let token = Component::new(token, None);

    parse_expression(token)
}

fn parse_conditional_operator(token: &Token) -> Component {
    let token = Component::new(token, None);

    parse_expression(token)
}

// comma operator
// expression list

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

fn parse_increment(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Increment", Some(token));

    parse_expression(component)
}

fn parse_decrement(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Decrement", Some(token));

    parse_expression(component)
}

fn parse_predefined_constant(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Predefined Constant", Some(token));

    parse_expression(component)
}

fn parse_primary_expression(token: Component) -> Component {
    let component = Component::new("Primary Expression", Some(token));
    
    parse_expression(component)
}

// -----------------------------------------
fn parse_allocation(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Memory Allocation", Some(token));

    parse_expression(component)
}

fn parse_scope_resolution(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Scope Resolution", Some(token));

    parse_nested_specifier(component)
}

fn parse_nested_specifier(token: Component) -> Component {
    let component = Component::new("Nested Specifier", Some(token));

    parse_expression(component)
}

fn parse_expression(token: Component) -> Component {
    let component = Component::new("Expression", Some(token));
    
    component
}
// -----------------------------------------
// Java Parsing
fn parse_super(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Abstract Data Type", Some(token));

    parse_primary_expression(component)
}

fn parse_this(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Abstract Data Type", Some(token));

    parse_primary_expression(component)
}

fn parse_abstract_data_type(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Abstract Data Type", Some(token));

    parse_data_type(component)
}

fn parse_modifier(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Modifier", Some(token));

    parse_declaration(component)
}

fn parse_try_block(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Try Block", Some(token));

    parse_statement(component)
}

fn parse_throw(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Throw Statement", Some(token));

    parse_statement(component)
}

fn parse_var(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Var Data Type", Some(token));

    parse_data_type(component)
}

fn parse_class_extension(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Class Extension", Some(token));

    parse_declaration(component)
}

fn parse_compilation_unit(token: &Token) -> Component {
    let token = Component::new(token, None);
    let component = Component::new("Compilation Unit", Some(token));

    component
}