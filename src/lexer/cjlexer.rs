use crate::prelude::*;

#[derive(Logos, Debug, Clone, Serialize, Deserialize, Eq, Ord, Hash, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Token {
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
    // declarator
    // ARRAY
    // FUNCTION
    // IDENTIFIER
    // POINTER
    // TYPEDEFNAME
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
    Asterisk,
    Multiplication,
    Pointer,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    // assignment
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
    #[token("&")]
    Ampersand,
    BitwiseAnd,
    Reference,
    #[token("|")]
    BitwiseOr,
    #[token("^")]
    BitwiseXor,
    #[token("~")]
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
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
    // sizeof
    #[token("sizeof")]
    SizeOf,
    // initialization
    // TODO: implement
    // DesignatedInitializer,
    // EqualsInitialization
    // InitializationList
    // FunctionCall
    #[regex(r"const_cast|static_cast|reinterpret_cast")]
    TypeCast,
    #[token("?")]
    ConditionalOperator,
    // CommaOperator
    // ExpressionList
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
    // memory allocation
    #[token("new")]
    New,
    #[token("delete")]
    Delete,
    // scope resolution
    #[token("::")]
    ScopeResolution,
    // predefined constant
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r"null|Null")]
    Null,

    #[token("using")]
    Using,
    #[token("namespace")]
    Namespace,
    // --------------------------------------------------------
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
    // --------------------------------------------------------
    
    #[regex("[A-Za-z_]+[A-Za-z0-9_]*", to_string)]
    Identifier(String),
    Function,
    Array,

    #[regex(r"[.0-9]+", to_string)]
    Number(String),
    #[token(";")]
    Semicolon,
    #[error]
    #[regex(r"[ \t]+", logos::skip)]
    Error,
    #[regex(r"[\n\r]+")]
    LineBreak,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    // find error
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,

    // #[regex(r"[A-Za-z0-9_]+\[[A-Za-z0-9_, ]*\]")]
    // Array,
    // #[regex(r"[A-Za-z0-9_]+\([A-Za-z0-9_, ]*\)")]
    // Function,
    // #[regex(r"\*[A-Za-z_]+[A-Za-z0-9_]*")]
    // Pointer,
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