/// A type-alias that represents the start and end point of a token.
pub type Span = (usize, usize);

/// Represents the "kind" of a token.
/// 
/// They are separated into groups in the source code, where each group signifies a sub-type of token.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Fn,
    Let,
    If,
    Else,
    While,
    Return,
    Break,
    Continue,
    Use,
    From,

    True,
    False,

    Identifier(String),
    String(String),
    Number(f64),

    Colon,
    DoubleColon,
    SemiColon,
    Comma,
    Dot,
    Arrow,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    DoubleAsterisk,

    And,
    Or,

    Equals,
    EqualsEquals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    Not,

    Eof,
}

/// Stores information regarding a token.
/// 
/// The `Token` type holds information about the type of a token (`TokenType`), as well as it's `line` and `span` (start and end column) in the source code.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, span: Span) -> Self {
        Self { kind, line, span }
    }

    pub fn eof() -> Self {
        Self {
            kind: TokenKind::Eof,
            line: 0,
            span: (0, 0),
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::Eof,
            line: 0,
            span: (0, 0),
        }
    }
}