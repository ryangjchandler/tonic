use std::iter::Peekable;
use std::str::Chars;
use crate::{Token, TokenKind, Span};

/// The main `Lexer` that handles producing `Token` instances.
/// 
/// Keeps track of the current column and holds the current character in memory.
#[derive(Debug)]
pub struct Lexer<'l> {
    source: Peekable<Chars<'l>>,
    line: usize,
    column: usize,
    current: char,
}

impl<'l> Lexer<'l> {

    /// Create a new `Lexer`.
    /// 
    /// The `source` is converted into a `Peekable<Chars<'l>>` to make tracking current
    /// column position easier.
    pub fn new(source: &'l str) -> Self {
        let mut this = Self {
            source: source.chars().peekable(),
            column: 0,
            line: 1,
            current: '\0',
        };

        this.read();
        this
    }

    /// Move the character set one column over and read in the next character.
    /// 
    /// Also increments line and column counters to keep track of location.
    fn read(&mut self) -> Option<char> {
        if let Some(c) = self.source.next() {
            self.current = c;
            self.column += 1;

            if c == '\n' {
                self.line += 1;
                self.column = 0;
            }

            Some(self.current)
        } else {
            self.current = '\0';

            None
        }
    }

    /// Skip over whitespace characters.
    fn skip_whitespace(&mut self) {
        while self.current.is_ascii_whitespace() {
            self.read();
        }
    }

    fn parse_identifier_or_keyword(&mut self) -> Token {
        let position = self.pos();
        let mut buffer = String::from(self.current);

        loop {
            if self.read().is_none() {
                break;
            }

            if ! is_valid_identifier_char(self.current) {
                break;
            }

            buffer.push(self.current);
        }

        if let Some(kind) = keyword(&buffer) {
            Token::new(kind, position.0, (position.1, self.column))
        } else {
            Token::new(TokenKind::Identifier(buffer), position.0, (position.1, self.column))
        }
    }

    fn parse_symbol(&mut self) -> Token {
        let position = self.pos();
        let buffer = String::from(self.current);

        self.read();

        if ! is_valid_symbol_char(self.current) {
            return Token::new(symbol(&buffer).unwrap(), position.0, (position.1, self.column))
        }

        let mut multi = String::from(&buffer);
        multi.push(self.current);

        if symbol(&multi).is_some() {
            self.read();

            return Token::new(symbol(&multi).unwrap(), position.0, (position.1, self.column))
        }

        Token::new(symbol(&buffer).unwrap(), position.0, (position.1, self.column))
    }

    fn parse_numeric(&mut self) -> Token {
        let position = self.pos();
        let mut buffer = String::from(self.current);

        loop {
            if self.read().is_none() {
                break;
            }

            if ! self.current.is_numeric() && self.current != '.' {
                break;
            }

            if self.current == '.' && buffer.contains('.') {
                panic!("Cannot have more than 1 `.` in a numeric value.");
            }

            if self.current == '.' {
                self.read();
                buffer.push('.');
            }

            buffer.push(self.current);
        }

        // We can safely unwrap the value here as we escape from the loop early 
        // when a non-numeric value is encountered.
        Token::new(TokenKind::Number(buffer.parse().unwrap()), position.0, (position.1, self.column))
    }

    fn parse_string(&mut self) -> Token {
        let position = self.pos();
        let mut buffer = String::new();
        let mut escaping = false;

        loop {
            if self.read().is_none() {
                break;
            }

            if escaping {
                buffer.push(match self.current {
                    't' => '\t',
                    'n' => '\n',
                    'r' => '\r',
                    _ => self.current
                });

                escaping = false;
                continue;
            }

            if is_valid_string_wrapper(self.current) {
                // Skip over the `"` character.
                self.read();

                break;
            }

            if self.current == '\\' {
                escaping = true;

                continue;
            }

            buffer.push(self.current)
        }

        Token::new(TokenKind::String(buffer), position.0, (position.1, self.column))
    }

    fn pos(&self) -> Span {
        (self.line, self.column)
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        Some(match self.current {
            _ if is_valid_identifier_char(self.current) => self.parse_identifier_or_keyword(),
            _ if is_valid_symbol_char(self.current) => self.parse_symbol(),
            _ if self.current.is_numeric() => self.parse_numeric(),
            _ if is_valid_string_wrapper(self.current) => self.parse_string(),
            _ => return None
        })
    }
}

fn is_valid_string_wrapper(c: char) -> bool {
    c == '"'
}

fn is_valid_symbol_char(c: char) -> bool {
    ['+', '-', '*', '/', '%', '{', '}', '(', ')', '[', ']', ':', ';', ',', '=', '!', '>', '<', '.', '&', '|'].contains(&c)
}

fn is_valid_identifier_char(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '$'
}

fn symbol(s: &str) -> Option<TokenKind> {
    Some(match s {
        "+" => TokenKind::Plus,
        "-" => TokenKind::Minus,
        "*" => TokenKind::Asterisk,
        "/" => TokenKind::Slash,
        "%" => TokenKind::Percent,
        "**" => TokenKind::DoubleAsterisk,
        "(" => TokenKind::LeftParen,
        ")" => TokenKind::RightParen,
        "{" => TokenKind::LeftBrace,
        "}" => TokenKind::RightBrace,
        "[" => TokenKind::LeftBracket,
        "]" => TokenKind::RightBracket,
        ":" => TokenKind::Colon,
        "::" => TokenKind::DoubleColon,
        ";" => TokenKind::SemiColon,
        "," => TokenKind::Comma,
        "=" => TokenKind::Equals,
        "==" => TokenKind::EqualsEquals,
        "!=" => TokenKind::NotEquals,
        ">" => TokenKind::GreaterThan,
        ">=" => TokenKind::GreaterThanEquals,
        "<" => TokenKind::LessThan,
        "<=" => TokenKind::LessThanEquals,
        "." => TokenKind::Dot,
        "!" => TokenKind::Not,
        "&&" => TokenKind::And,
        "||" => TokenKind::Or,
        _ => return None
    })
}

fn keyword(s: &str) -> Option<TokenKind> {
    Some(match s {
        "fn" => TokenKind::Fn,
        "let" => TokenKind::Let,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "return" => TokenKind::Return,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "use" => TokenKind::Use,
        "from" => TokenKind::From,
        _ => return None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords() {
        matches("fn if else while return break continue let true false use from", vec![
            TokenKind::Fn,
            TokenKind::If,
            TokenKind::Else,
            TokenKind::While,
            TokenKind::Return,
            TokenKind::Break,
            TokenKind::Continue,
            TokenKind::Let,
            TokenKind::True,
            TokenKind::False,
            TokenKind::Use,
            TokenKind::From,
        ]);
    }

    #[test]
    fn symbols() {
        matches("+ - * / % ** ( ) { } [ ] : :: ; , = == != > < >= <= . !", vec![
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Asterisk,
            TokenKind::Slash,
            TokenKind::Percent,
            TokenKind::DoubleAsterisk,
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::LeftBracket,
            TokenKind::RightBracket,
            TokenKind::Colon,
            TokenKind::DoubleColon,
            TokenKind::SemiColon,
            TokenKind::Comma,
            TokenKind::Equals,
            TokenKind::EqualsEquals,
            TokenKind::NotEquals,
            TokenKind::GreaterThan,
            TokenKind::LessThan,
            TokenKind::GreaterThanEquals,
            TokenKind::LessThanEquals,
            TokenKind::Dot,
            TokenKind::Not,
        ]);
    }

    #[test]
    fn numbers() {
        matches("12345 12345.6789 9876.0", vec![
            TokenKind::Number(12345.0),
            TokenKind::Number(12345.6789),
            TokenKind::Number(9876.0),
        ]);
    }

    #[test]
    fn strings() {
        matches(r##""hello" "hello\"" "hello\n""##, vec![
            TokenKind::String("hello".into()),
            TokenKind::String("hello\"".into()),
            TokenKind::String("hello\n".into()),
        ]);
    }

    fn matches(source: &str, expected: Vec<TokenKind>) {
        let mut lexer = Lexer::new(source); 
        let mut kinds = Vec::new();

        while let Some(t) = lexer.next() {
            kinds.push(t.kind);
        }

        assert_eq!(expected, kinds)
    }
}