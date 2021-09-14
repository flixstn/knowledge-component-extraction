use crate::{
    lexer::lexer::Token,
    parser::Parser,
    parser::knowledge_component::{KnowledgeComponent, Component},
};
use indexmap::IndexSet;
use logos::Logos;
use serde::{Serialize, Deserialize};
use std::error::Error;

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
        let components = &mut self.knowledge_components;
        let mut token_iter = tokens.iter().enumerate();
        let time_stamp = format!("{}&t={}", self.source, time_code);

        while let Some((idx, token)) = token_iter.next() {
            match token {
                // Preprocessor Classification
                // preprocessor
                Token::Preprocessor(ident) => {
                    let classification = parse_preprocessor(&token);
                    let component = KnowledgeComponent::new_with_ident(classification, "Preprocessor", ident, &time_stamp);
                    components.insert(component);
                }
                // Statement Classification
                // iteration
                Token::For => {
                    let classification = parse_iteration(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::While => {
                    let classification = parse_iteration(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::DoWhile => {
                    let classification = parse_iteration(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // selection
                Token::Switch => {
                    let classification = parse_condition(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Case => {
                    let classification = parse_label(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Default => {
                    let classification = parse_label(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::If => {
                    let classification = parse_condition(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::ElseIf => {
                    let classification = parse_condition(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Else => {
                    let classification = parse_condition(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // jump
                Token::Break => {
                    let classification = parse_jump(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Continue => {
                    let classification = parse_jump(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Goto => {
                    let classification = parse_jump(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Return => {
                    let classification = parse_jump(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // Declaration Classification
                // arithmetic type
                Token::Unsigned => {
                    let classification = parse_sign(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Signed => {
                    let classification = parse_sign(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);   
                }
                Token::Char => {
                    let classification = parse_character(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Int => {
                    let classification = parse_integer(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Short => {
                    let classification = parse_integer(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::ShortInt => {
                    let classification = parse_integer(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Long => {
                    let classification = parse_integer(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::LongInt => {
                    let classification = parse_integer(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::LongLong => {
                    let classification = parse_integer(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::LongLongInt => {
                    let classification = parse_integer(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Float => {
                    let classification = parse_float(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Double => {
                    let classification = parse_float(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::LongDouble => {
                    let classification = parse_float(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Bool => {
                    let classification = parse_boolean(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::String => {
                    let classification = parse_string(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // elaborated type specifier
                Token::Enum => {
                    let classification = parse_elaborated_type_specifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Struct => {
                    let classification = parse_elaborated_type_specifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Union => {
                    let classification = parse_elaborated_type_specifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Class => {
                    let classification = parse_elaborated_type_specifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Template => {
                    // TODO: Data Type -> Abstract Data Type -> Data Type -> Statement
                }
                Token::Void => {
                    let classification = parse_void(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);    
                }

                // TODO: implement Declarators!-----------
                // ARRAY
                Token::Array => {
                    let classification = parse_declarator(&Token::Array);
                    let component = KnowledgeComponent::new(classification, &Token::Array, &time_stamp);
                    components.insert(component);
                }
                Token::Function => {
                    let peek = tokens.get(idx-1).unwrap_or(&Token::Error);
                    println!("{:?}", peek);
                    match *peek {
                        // check whether token is function
                        Token::Int | Token::Short | Token::Long | Token::Float | Token::Double |
                        Token::Bool | Token::String | Token::Volatile | Token::Const | Token::Restrict => {
                            let classification = parse_declarator(&Token::Function);
                            let component = KnowledgeComponent::new(classification, &Token::Function, &time_stamp);
                            components.insert(component);
                        }
                        // check whether token is function call
                        _ => {
                            let classification = parse_function_call(&Token::FunctionCall);
                            let component = KnowledgeComponent::new(classification, &Token::FunctionCall, &time_stamp);
                            components.insert(component);
                        }
                    }
                    // check whether token is function call
                    let classification = parse_declarator(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                },
                // IDENTIFIER       // VARIABLE
                // POINTER
                // TYPEDEFNAME
                // --------------------------------------- 

                // storage class
                Token::Auto => {
                    let classification = parse_storage_class(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);    
                }
                Token::Extern => {
                    let classification = parse_storage_class(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                Token::Register => {
                    let classification = parse_storage_class(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                Token::Static => {
                    let classification = parse_storage_class(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                Token::ThreadLocal => {
                    let classification = parse_storage_class(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                Token::Typedef => {
                    let classification = parse_storage_class(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                // TODO: check CPP Grammar for correct implementation
                Token::Decltype => {
                    let classification = parse_storage_class(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                // type qualifier
                Token::Atomic => {
                    let classification = parse_type_qualifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                Token::Const => {
                    let classification = parse_type_qualifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Restrict => {
                    let classification = parse_type_qualifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Volatile => {
                    let classification = parse_type_qualifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Mutable => {
                    let classification = parse_type_qualifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Public => {
                    let classification = parse_access_specifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Protected => {
                    let classification = parse_access_specifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Private => {
                    let classification = parse_access_specifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // Expression Declaration
                // arithmetic
                Token::Plus => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Minus => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // TODO: implement MUL and PTR
                Token::Multiplication => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);       
                }
                Token::Divide => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Modulo => {
                    let classification = parse_arithmetic(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // assignment
                Token::AddAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::SubAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::MultAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::DivAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::ModAssignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Assignment => {
                    let classification = parse_assignment(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // bitwise
                Token::Ampersand => {
                    let peek = tokens.get(idx-1).unwrap_or(&Token::Error);
                    match *peek {
                        Token::Number(_) | Token::Identifier(_) => {
                            let classification = parse_bitwise(&Token::BitwiseAnd);
                            let component = KnowledgeComponent::new(classification, &Token::BitwiseAnd, &time_stamp);
                            components.insert(component);
                        }
                        _ => {
                            let classification = parse_member_access(&Token::Reference);
                            let component = KnowledgeComponent::new(classification, &Token::Reference, &time_stamp);
                            components.insert(component);
                        }
                    }
                }
                Token::BitwiseAnd => {}
                Token::Reference => {}
                Token::BitwiseOr => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseXor => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseNot => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::LeftOperator => {
                    let peek = tokens.get(idx-1).unwrap_or(&Token::Error);
                    match &*peek {
                        // check whether lhs is identifier or cout
                        Token::Identifier(ident) => {
                            match ident.as_str() {
                                "cout" | "cerr" | "clog" => {
                                    continue;
                                }
                                _ => {
                                    let classification = parse_bitwise(&Token::LeftShift);
                                    let component = KnowledgeComponent::new(classification, &Token::LeftShift, &time_stamp);
                                    components.insert(component);
                                }
                            }
                        }
                        // check whether lhs is Token::Number
                        Token::Number(_) => {
                            let classification = parse_bitwise(&Token::LeftShift);
                            let component = KnowledgeComponent::new(classification, &Token::LeftShift, &time_stamp);
                            components.insert(component);
                        }
                        _ => ()
                    }     
                }
                Token::LeftShiftAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, &Token::RightShift, &time_stamp);
                    components.insert(component);
                }
                Token::RightOperator => {
                    let peek = tokens.get(idx-1).unwrap_or(&Token::Error);
                    // check whether lhs is identifier or cin
                    match &*peek {
                        Token::Identifier(ident) => {
                            if let "cin" = ident.as_str() {
                                continue;
                            }
                            let classification = parse_bitwise(&Token::RightShift);
                            let component = KnowledgeComponent::new(classification, &Token::RightShift, &time_stamp);
                            components.insert(component);
                        }
                        _ => {
                            let classification = parse_bitwise(&Token::RightShift);
                            let component = KnowledgeComponent::new(classification, &Token::RightShift, &time_stamp);
                            components.insert(component);
                        }
                    }
                }
                Token::RightShiftAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, &Token::RightShift, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseAndAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseOrAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::BitwiseXorAssignment => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // logical
                Token::And => {
                    let classification = parse_logical(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Or => {
                    let classification = parse_logical(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Not => {
                    let classification = parse_logical(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // sizeof
                Token::SizeOf => {
                    let classification = parse_size_of(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // initializiation
                Token::DesignatedInitializer => {
                    let classification = parse_initialization(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // function call
                Token::FunctionCall => {}
                Token::TypeCast => {
                    let classification = parse_type_cast(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::ConditionalOperator => {
                    let classification = parse_conditional_operator(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // comparison
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
                Token::Equals => {
                    let classification = parse_comparison(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::NotEquals => {
                    let classification = parse_comparison(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::ThreeWayComparison => {
                    let classification = parse_comparison(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);                   
                }
                // member access
                Token::DotOperator => {
                    let classification = parse_member_access(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::ArrowOperator => {
                    let classification = parse_member_access(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // increment decrement
                Token::Increment => {
                    let peek = tokens.get(idx+1).unwrap_or(&Token::Error);
                    match &*peek {
                        // check whether rhs is variable | number
                        Token::Identifier(_ident) | Token::Number(_ident) => {
                            let classification = parse_increment(&Token::PrefixIncrement);
                            let component = KnowledgeComponent::new(classification, &Token::PrefixIncrement, &time_stamp);
                            components.insert(component);
                        }
                        // check whether lhs is variable | number
                        _ => {
                            let classification = parse_increment(&Token::PostfixIncrement);
                            let component = KnowledgeComponent::new(classification, &Token::PostfixIncrement, &time_stamp);
                            components.insert(component); 
                        }
                    }
                }
                Token::Decrement => {
                    let peek = tokens.get(idx+1).unwrap_or(&Token::Error);
                    match &*peek {
                        // check whether rhs is variable | number
                        Token::Identifier(_ident) | Token::Number(_ident) => {
                            let classification = parse_decrement(&Token::PrefixDecrement);
                            let component = KnowledgeComponent::new(classification, &Token::PrefixDecrement, &time_stamp);
                            components.insert(component);
                        }
                        // check whether lhs is variable | number
                        _ => {
                            let classification = parse_decrement(&Token::PostfixDecrement);
                            let component = KnowledgeComponent::new(classification, &Token::PostfixDecrement, &time_stamp);
                            components.insert(component); 
                        }
                    }
                }
                // TODO: primary expression

                // ------------------------------------------
                Token::New => {
                    let classification = parse_allocation(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);      
                }
                Token::Delete => {
                    let classification = parse_allocation(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);      
                }
                Token::ScopeResolution => todo!(),
                // ------------------------------------------
                Token::Number(_ident) => {

                }
                Token::Identifier(_ident) => {
                    let classification = parse_declarator(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);    
                },
                Token::Error => {},
                Token::StringDoubleQuoted(_ident) => {}
                Token::LeftShift => todo!(),
                Token::RightShift => todo!(),
                Token::PrefixIncrement => todo!(),
                Token::PostfixIncrement => todo!(),
                Token::PrefixDecrement => todo!(),
                Token::PostfixDecrement => todo!(),
                Token::Using => todo!(),
                Token::Namespace => todo!(),
                Token::Comment => todo!(),
                Token::True => {
                    let classification = parse_predefined_constant(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::False => {
                    let classification = parse_predefined_constant(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::Pointer => {},
                Token::TypedefName => {},
                Token::Null => {
                    let classification = parse_predefined_constant(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);    
                }
                // Compilation Unit
                Token::Import => {
                    let classification = parse_compilation_unit(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                },
                Token::Package => {
                    let classification = parse_compilation_unit(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                // Modifier
                Token::Annotation => {
                    let classification = parse_modifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::Abstract => {
                    let classification = parse_modifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::Final => {
                    let classification = parse_modifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::Native => {
                    let classification = parse_modifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::Synchronized => {
                    let classification = parse_modifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::Transient => {
                    let classification = parse_modifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::StrictFp => {
                    let classification = parse_modifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::Assert => {
                    let classification = parse_modifier(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                // Abstract Data Type
                Token::Interface => {
                    let classification = parse_abstract_data_type(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                // Try/Catch
                Token::Try => {
                    let classification = parse_try_block(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Catch => {
                    let classification = parse_try_block(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Finally => {
                    let classification = parse_try_block(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // Throw
                Token::Throws => {
                    let classification = parse_throw(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                Token::Throw => {
                    let classification = parse_throw(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // Bitwise
                Token::UnsignedRightShiftOperator => {
                    let classification = parse_bitwise(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);
                }
                // Primary Expression
                Token::Super => {
                    let classification = parse_super(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component);  
                }
                Token::This => {
                    let classification = parse_this(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                Token::Extends => {
                    let classification = parse_class_extension(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                Token::Implements => {
                    let classification = parse_class_extension(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }
                Token::Var => {
                    let classification = parse_var(&token);
                    let component = KnowledgeComponent::new(classification, token, &time_stamp);
                    components.insert(component); 
                }

                // _ => ()
            }
        }
        
        Ok(())
    }
}

// Preprocessor -----------------------------------------------------
fn parse_preprocessor(token: &Token) -> Component {
    let token = Component::new(token.to_string().replace("\"", ""), None);
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

// TODO: member access
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