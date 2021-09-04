use logos::{Logos, Lexer};
use serde::{Serialize, Deserialize};
use std::hash::Hash;

#[derive(Logos, Debug, Clone, Serialize, Deserialize, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub enum Token {
    // iteration
    #[token("for")]
    #[token("while")]
    // jump
    #[token("break")]
    #[token("continue")]
    #[token("return")]
    #[token("pass")]
    // selection
    #[token("match")]
    #[token("case")]
    #[token("if")]
    #[token("elif")]
    #[token("else")]
    // data type
    // #[regex(r"[+-]?([0-9]*[.])?[0-9]+")]
    #[regex(r"[0-9]*[.][0-9]+")]
    #[token("float")]
    Float,
    #[regex(r"[0-9]+")]
    #[token("int")]
    Int,
    #[regex(r"[0-9]+j")]
    #[token("complex")]
    Complex,
    #[regex("True")]
    True,
    #[regex("False")]
    False,
    #[token("bool")]
    Bool,
    #[token("list")]
    #[regex(r"\[[a-zA-Z0-9,]*\]")]
    List,
    #[token("tuple")]
    #[regex(r"\([a-zA-Z0-9,]*\)")]
    // #[regex(r"([a-zA-Z0-9],)*")]
    Tuple,
    #[token("dict")]
    #[regex(r"\{[a-zA-Z0-9,:]*\}")]
    Dict,
    #[token("set")]
    Set,
    #[token("bytes")]
    Bytes,
    #[regex("class")]
    Class,
    #[regex("\"(?s:[^\"\\\\]|\\\\.)*\"|'(?s:[^'\\\\]|\\\\.)*'")]    // #[regex("\"(?s:[^\"\\\\]|\\\\.)*\"", to_string)]
    String,
    #[token("None")]
    None,
    // declarator
    #[token("def")]
    FunctionDefinition,
    #[token("@")]
    Decorator,
    #[token("@final")]
    Final,
    #[token("@overload")]
    Overload,
    // type qualifier
    #[token("global")]
    Global,
    // arithmetic
    #[token("+")]
    Addition,
    #[token("+=")]
    AddAssignment,
    #[token("-")]
    Subtraction,
    #[token("-=")]
    SubAssignment,
    #[token("*")]
    Multiplication,
    #[token("*=")]
    MultAssignment,
    #[token("/")]
    Division,
    #[token("/=")]
    DivAssignment,
    #[token("//")]
    FloorDivision,
    #[token("//=")]
    FloorDivAssignment,
    #[token("%")]
    Modulo,
    #[token("%=")]
    ModAssignment,
    #[token("**")]
    Exponentiation,
    #[token("**=")]
    ExpAssignment,
    #[token("=")]
    Assignment,
    #[token(":=")]
    AssignmentExpression,
    // bitwise
    #[token("&")]
    BitwiseAnd,
    #[token("&=")]
    BitwiseAndAssignment,
    #[token("|")]
    BitwiseOr,
    #[token("|=")]
    BitwiseOrAssignment,
    #[token("^")]
    BitwiseXor,
    #[token("^=")]
    BitwiseXorAssignment,
    #[token("~")]
    BitwiseNot,
    #[token("<<")]
    BitwiseLeftShift,
    #[token("<<=")]
    BitwiseLeftShiftAssignment,
    #[token(">>")]
    BitwiseRightShift,
    #[token(">>=")]
    BitwiseRightShiftAssignment,
    // function call
    #[regex(r"[A-Za-z0-9_]+\([A-Za-z0-9_, ]*\)")]
    Function,
    // logical
    #[token("and")]
    LogicalAnd,
    #[token("or")]
    LogicalOr,
    #[token("not")]
    LogicalNot,
    // comparison
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterOrEquals,
    #[token("<")]
    Less,
    #[token("<=")]
    LessOrEquals,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEquals,
    // member access
    #[token(".")]
    MemberAccess,
    // identity operator
    #[token("is")]
    Is,
    #[token("is not")]
    IsNot,
    // membership operator
    #[token("in")]
    In,
    #[token("not in")]
    NotIn,
    // yield expression
    #[token("yield")]
    Yield,
    // async expression
    #[token("async")]
    Async,
    // await expression
    #[token("await")]
    Await,
    // lambda expression
    #[token("lambda")]
    Lambda,


    #[regex(r"#[^\r\n]*(\r\n|\n)?")]
    Comment,
    #[error]
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Error,

    
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&Token> for String {
    fn from(token: &Token) -> Self {
        token.to_string()
    }
}

fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice().to_string();
    Some(slice)
}
