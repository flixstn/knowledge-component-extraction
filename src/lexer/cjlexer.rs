use logos::{Logos, Lexer};
use serde::{Serialize, Deserialize};
use std::hash::Hash;

#[derive(Logos, Debug, Clone, Serialize, Deserialize, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Token {
    // preprocessor
    #[regex(r"#define|#undef|#ifdef|#ifndef|#if|#endif|#else|#elif|#line|#error|#include|#pragma", to_string)]
    Preprocessor(String),
    // statement
    // iteration
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("do")]
    DoWhile,
    // selection/condition
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("if")]
    If,
    #[regex(r"else if|elif")]
    ElseIf,
    #[token("else")]
    Else,
    // jump
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("goto")]
    Goto,
    #[token("return")]
    Return,
    // declaration
    // arithmetic type
    #[token("unsigned")]
    Unsigned,
    #[token("signed")]
    Signed,
    #[regex("char|char8_t|char16_t|char32_t|wchar_t")]
    Char,
    #[token("int")]
    Int,
    #[token("short")]
    Short,
    #[token("short int")]
    ShortInt,
    #[token("long")]
    Long,
    #[token("long int")]
    LongInt,
    #[token("long long")]
    LongLong,
    #[token("long long int")]
    LongLongInt,
    #[token("float")]
    Float,
    #[token("double")]
    Double,
    #[token("long double")]
    LongDouble,
    #[regex("bool|boolean")]
    Bool,
    #[regex(r"String|string")]
    String,
    // -> elaborated type specifier
    #[token("enum")]
    Enum,
    #[token("struct")]
    Struct,
    #[token("union")]
    Union,
    #[token("class")]
    Class,
    #[token("template")]
    Template,
    // void
    #[token("void")]
    Void,
    //declarator
    #[regex(r"[A-Za-z0-9_]+\[[A-Za-z0-9_, ]*\]")]
    Array,
    #[regex(r"[A-Za-z0-9_]+\([A-Za-z0-9_, ]*\)")]
    Function,
    #[regex("[A-Za-z_]+[A-Za-z0-9_]*", to_string)]
    Identifier(String),
    #[regex(r"\*[A-Za-z_]+[A-Za-z0-9_]*")]
    Pointer,
    TypedefName,
    // storage class
    #[token("auto")]
    Auto,
    #[token("extern")]
    Extern,
    #[token("register")]
    Register,
    #[token("static")]
    Static,
    #[regex("_Thread_local|thread_local")]
    ThreadLocal,
    #[token("typedef")]
    Typedef,
    #[token("decltype")]
    Decltype,
    // type qualifier
    #[regex("_Atomic|atomic_int")]
    Atomic,
    #[token("const")]
    Const,
    #[token("restrict")]
    Restrict,
    #[token("volatile")]
    Volatile,
    #[token("mutable")]
    Mutable,
    #[token("public")]
    Public,
    #[token("protected")]
    Protected,
    #[token("private")]
    Private,
    // expression
    // arithmetic
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiplication,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("+=")]
    AddAssignment,
    #[token("-=")]
    SubAssignment,
    #[token("*=")]
    MultAssignment,
    #[token("/=")]
    DivAssignment,
    #[token("%=")]
    ModAssignment,
    #[token("=")]
    Assignment,
    // bitwise
    #[token("&")]            // #[regex("-?[0-9]+\\.[0-9]+", priority=2)] // #[regex("[a-zA-Z0-9_]+\s*\&\s*[a-zA-Z0-9_]+")]
    Ampersand,
    BitwiseAnd,
    Reference,
    #[token("|")]           // #[regex(r"[a-zA-Z0-9_]+\s*\|\s*[a-zA-Z0-9_]+")]
    BitwiseOr,
    #[token("^")]           // #[regex(r#"[a-zA-Z0-9_]+\s*\^\s*[a-zA-Z0-9_]+"#)]
    BitwiseXor,
    #[token("~")]           // #[regex(r#"~\s*[a-zA-Z0-9_]+"#)]
    BitwiseNot,
    #[token("<<")]
    LeftOperator,
    LeftShift,
    #[token("<<=")]
    LeftShiftAssignment,
    #[token(">>")]
    RightOperator,
    RightShift,
    #[token(">>=")]
    RightShiftAssignment,
    #[token("&=")]
    BitwiseAndAssignment,
    #[token("|=")]
    BitwiseOrAssignment,
    #[token("^=")]
    BitwiseXorAssignment,
    // logical
    #[token("&&")]          // #[regex(r"[a-zA-Z0-9_]+\s*\&\&\s*[a-zA-Z0-9_]+")]
    And,
    #[token("||")]          // #[regex(r"[a-zA-Z0-9_]+\s*\|\|\s*[a-zA-Z0-9_]+")]
    Or,
    #[token("!")]           // #[regex(r"!\s*[a-zA-Z0-9_]+")]
    Not,
    #[token("sizeof")]                          // size_of_expression || size_of_type
    SizeOf,
    // initialization
    #[regex(r"[A-Za-z_]+[A-Za-z0-9_]* *\{")]
    DesignatedInitializer,
    // EqualsInitialization
    // InitializationList
    FunctionCall,
    #[regex(r"const_cast|static_cast|reinterpret_cast")]
    TypeCast,
    #[token("?")]
    ConditionalOperator,
    // TODO: CommaOperator
    // TODO: ExpressionList
    #[token("using")]
    Using,
    #[token("namespace")]
    Namespace,
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
    Equals,
    #[token("!=")]
    NotEquals,
    #[token("<=>")]
    ThreeWayComparison,
    // member access
    // ArraySubscript
    #[token(".")]
    DotOperator,
    // PointerDereference
    #[token("->")]
    ArrowOperator,
    // increment decrement
    #[token("++")]
    Increment,
    PrefixIncrement,
    PostfixIncrement,
    #[token("--")]
    Decrement,
    PrefixDecrement,
    PostfixDecrement,
    //primary expression
    // ----------------------------------
    
    // memory allocation
    #[token("new")]
    New,
    #[token("delete")]
    Delete,
    // scope resolution
    #[token("::")]
    ScopeResolution,
    
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r"null|Null")]
    Null,

    #[regex(r"[.0-9]+", to_string, priority=11)] // #[regex(r"[0-9]+", priority=2)] // #[regex(r"[.0-9]+", priority=2)]
    Number(String),
    #[regex("\"(?s:[^\"\\\\]|\\\\.)*\"", to_string)]
    StringDoubleQuoted(String),
    // TODO: Line Comment
    #[regex(r"//[^\r\n]*(\r\n|\n)?")]
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    Comment,
    #[error]
    #[regex(r"[ \t]+", logos::skip)]
    // #[regex(r"[ \t\n\r]+", logos::skip)]
    Error,

    #[regex(r"[\n\r]+")]
    LineBreak,
    // --------------------------------------------------------
    // Java Parser
    #[token("import")]
    Import,
    #[token("@")]
    Annotation,
    #[token("interface")]
    Interface,
    #[token("abstract")]
    Abstract,
    #[token("final")]
    Final,
    #[token("native")]
    Native,
    #[token("synchronized")]
    Synchronized,
    #[token("transient")]
    Transient,
    #[token("strictfp")]
    StrictFp,
    #[token("assert")]
    Assert,
    #[token("try")]
    Try,
    #[token("catch")]
    Catch,
    #[token("throws")]
    Throws,
    #[token("throw")]
    Throw,
    #[token(">>>")]
    UnsignedRightShiftOperator,
    #[token("super")]
    Super,
    #[token("this")]
    This,
    #[token("extends")]
    Extends,
    #[token("implements")]
    Implements,
    #[token("package")]
    Package,
    #[token("finally")]
    Finally,
    #[token("var")]
    Var,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
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
