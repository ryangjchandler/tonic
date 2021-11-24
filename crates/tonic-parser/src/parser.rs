use crate::{Statement, Expression, Token, TokenKind, Lexer, Type, Parameter, Span};
use std::collections::HashMap;

pub type Program = Vec<Statement>;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub line: usize,
    pub span: Span,
    pub err: ParserErrorType,
}

#[derive(Debug, Clone)]
pub enum ParserErrorType {
    InvalidBreakableScope,
    InvalidContinuableScope,
    UnexpectedToken(String, Option<String>),
    NestedFunctionDefinition,
    ExpectedIdentifier,
}

type BindingPower = u8;
type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'p> {
    lexer: Lexer<'p>,
    current: Token,
    peek: Token,
    in_breakable_scope: bool,
    scope_depth: usize,
}

#[allow(dead_code)]
impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        Self {
            lexer,
            current: Token::eof(),
            peek: Token::eof(),
            in_breakable_scope: false,
            scope_depth: 0,
        }
    }

    pub fn read(&mut self) {
        self.current = std::mem::replace(&mut self.peek, if let Some(t) = self.lexer.next() { t } else { Token::eof() });
    }

    fn parse_statement(&mut self) -> ParserResult<Statement> {
        Ok(match self.current.kind {
            TokenKind::Let => self.parse_let()?,
            TokenKind::Fn => self.parse_fn()?,
            TokenKind::If => self.parse_if()?,
            TokenKind::While => self.parse_while()?,
            TokenKind::Use => {
                self.read();

                let mut imports = Vec::new();

                while self.current.kind != TokenKind::From {
                    if imports.len() > 0 {
                        self.expect(TokenKind::Comma)?;
                    }

                    let import = self.identifier()?;

                    imports.push(import);
                }

                self.expect(TokenKind::From)?;

                let module = self.string()?;

                Statement::Use {
                    module,
                    imports
                }
            },
            TokenKind::Break => {
                if ! self.in_breakable_scope {
                    return Err(ParserError { line: self.current.line, span: self.current.span, err: ParserErrorType::InvalidBreakableScope })
                }

                self.read();

                Statement::Break
            },
            TokenKind::Continue => {
                if ! self.in_breakable_scope {
                    return Err(ParserError { line: self.current.line, span: self.current.span, err: ParserErrorType::InvalidContinuableScope })
                }

                self.read();

                Statement::Continue
            },
            TokenKind::Return => {
                self.read();

                let expression = self.expression(0)?;

                Statement::Return { expression }
            },
            _ => {
                Statement::Expression { expression: self.expression(0)? }
            },
        })
    }

    fn parse_let(&mut self) -> ParserResult<Statement> {
        self.read();

        let identifier = self.identifier()?;
        let mut r#type = self.r#type()?;

        self.expect(TokenKind::Equals)?;

        let expression = self.expression(0)?;

        match expression {
            Expression::String(_) => r#type = Some(Type::String),
            Expression::Number(_) => r#type = Some(Type::Number),
            Expression::Bool(_) => r#type = Some(Type::Bool),
            _ => (),
        };

        Ok(Statement::Let { identifier, r#type, initial: expression })
    }

    fn parse_fn(&mut self) -> ParserResult<Statement> {
        if self.scope_depth > 0 {
            return Err(ParserError {
                line: self.current.line,
                span: self.current.span,
                err: ParserErrorType::NestedFunctionDefinition,
            });
        }

        self.scope_depth += 1;

        self.read();

        let identifier = self.identifier()?;

        self.expect(TokenKind::LeftParen)?;

        let parameters = self.parameters()?;

        self.expect(TokenKind::RightParen)?;

        let return_type = self.r#type()?;

        self.expect(TokenKind::LeftBrace)?;

        let body = self.block(TokenKind::RightBrace)?;

        self.expect(TokenKind::RightBrace)?;

        self.scope_depth -= 1;

        Ok(Statement::Function {
            identifier, parameters, return_type, body
        })
    }

    fn parse_if(&mut self) -> ParserResult<Statement> {
        self.read();

        let condition = self.expression(0)?;

        self.expect(TokenKind::LeftBrace)?;

        let then = self.block(TokenKind::RightBrace)?;

        self.expect(TokenKind::RightBrace)?;

        let mut otherwise = Vec::new();

        if self.current.kind == TokenKind::Else {
            self.read();

            // If we see another `if` token, then we're going to parse an `else if` statement.
            if self.current.kind == TokenKind::If {
                otherwise = vec![self.parse_if()?];
            } else {
                self.expect(TokenKind::LeftBrace)?;

                otherwise = self.block(TokenKind::RightBrace)?;

                self.expect(TokenKind::RightBrace)?;
            }
        }

        Ok(Statement::If { condition, then, otherwise })
    }

    fn parse_while(&mut self) -> ParserResult<Statement> {
        self.read();

        let condition = self.expression(0)?;

        self.expect(TokenKind::LeftBrace)?;
        
        self.in_breakable_scope = true;

        let then = self.block(TokenKind::RightBrace)?;

        self.expect(TokenKind::RightBrace)?;

        self.in_breakable_scope = false;

        Ok(Statement::While { condition, then })
    }

    fn expect(&mut self, kind: TokenKind) -> ParserResult<()> {
        if std::mem::discriminant(&kind) == std::mem::discriminant(&self.current.kind) {
            self.read();

            Ok(())
        } else {
            return Err(ParserError { line: self.current.line, span: self.current.span, err: ParserErrorType::UnexpectedToken(format!("{:?}", self.current.kind), Some(format!("{:?}", kind))) })
        }
    }

    fn expression(&mut self, bp: u8) -> ParserResult<Expression> {
        let mut lhs = match self.current.kind.clone() {
            TokenKind::Number(n) => {
                self.read();

                Expression::Number(n)
            },
            TokenKind::True => {
                self.read();

                Expression::Bool(true)
            },
            TokenKind::False => {
                self.read();

                Expression::Bool(false)
            },
            TokenKind::String(s) => {
                self.read();

                Expression::String(s)
            },
            TokenKind::Identifier(i) => {
                self.read();

                Expression::Identifier(i)
            },
            TokenKind::LeftBracket => {
                self.read();

                let mut items = Vec::new();

                while self.current.kind != TokenKind::RightBracket {
                    items.push(self.expression(0)?);

                    if self.current.kind == TokenKind::Comma {
                        self.read();
                    }
                }

                self.expect(TokenKind::RightBracket)?;

                Expression::Array(items)
            },
            TokenKind::LeftBrace => {
                self.read();

                let mut members = HashMap::new();

                while self.current.kind != TokenKind::RightBrace {
                    let key = self.string()?;

                    self.expect(TokenKind::Colon)?;

                    let value = self.expression(0)?;

                    members.insert(key, value);

                    if self.current.kind == TokenKind::Comma {
                        self.read();
                    }
                }

                self.expect(TokenKind::RightBrace)?;

                Expression::Map(members)
            },
            TokenKind::Fn => {
                self.expect(TokenKind::Fn)?;

                self.expect(TokenKind::LeftParen)?;

                let params = self.parameters()?;

                self.expect(TokenKind::RightParen)?;

                dbg!(&self.current.kind);

                let body = if self.current.kind == TokenKind::Arrow {
                    self.read();

                    let expression = self.expression(0)?;

                    vec![Statement::Return { expression }]
                } else {
                    self.expect(TokenKind::LeftBrace)?;
                    
                    let body = self.block(TokenKind::RightBrace)?;

                    self.expect(TokenKind::RightBrace)?;

                    body
                };

                Expression::Closure(params, body)
            },
            TokenKind::LeftParen => {
                self.expect(TokenKind::LeftParen)?;

                let expression = self.expression(0)?;

                self.expect(TokenKind::RightParen)?;

                expression
            },
            _ if is_prefix(&self.current.kind) => {
                let kind = self.current.kind.clone();

                self.read();

                let (_, rbp) = prefix_binding_power(&kind);

                let rhs = self.expression(rbp)?;

                prefix(&kind, rhs)
            },
            _ => todo!("{:?}", self.current.kind),
        };

        loop {
            if self.current.kind == TokenKind::Eof {
                break;
            }

            let op = self.current.kind.clone();

            if let Some((lbp, _)) = postfix_binding_power(&op) {
                if lbp < bp {
                    break;
                }

                self.read();

                lhs = postfix(self, lhs, &op)?;

                continue;
            }

            if let Some((lbp, rbp)) = infix_binding_power(&op) {
                if lbp < bp {
                    break;
                }

                self.read();

                let rhs = self.expression(rbp)?;

                lhs = infix(lhs, &op, rhs);

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn string(&mut self) -> ParserResult<String> {
        match self.current.kind.clone() {
            TokenKind::String(i) => {
                self.read();

                Ok(i)
            },
            _ => Err(ParserError { line: self.current.line, span: self.current.span, err: ParserErrorType::UnexpectedToken(format!("{:?}", self.current.kind), Some("String".to_owned())) })
        }
    }

    fn identifier(&mut self) -> ParserResult<String> {
        match self.current.kind.clone() {
            TokenKind::Identifier(i) => {
                self.read();

                Ok(i)
            },
            _ => Err(ParserError { line: self.current.line, span: self.current.span, err: ParserErrorType::ExpectedIdentifier })
        }
    }

    fn args(&mut self) -> ParserResult<Vec<Expression>> {
        let mut args = Vec::new();

        loop {
            if self.current.kind == TokenKind::RightParen {
                break;
            }

            let expression = self.expression(0)?;

            args.push(expression);

            if self.current.kind == TokenKind::Comma {
                self.read();
            }
        }

        Ok(args)
    }

    fn parameters(&mut self) -> ParserResult<Vec<Parameter>> {
        let mut parameters = Vec::new();

        loop {
            if self.current.kind == TokenKind::RightParen {
                break;
            }

            let identifier = self.identifier()?;
            let r#type = self.r#type()?;

            parameters.push(Parameter::new(identifier, r#type));

            if self.current.kind == TokenKind::Comma {
                self.read();
            }
        }

        Ok(parameters)
    }

    fn block(&mut self, end: TokenKind) -> ParserResult<Vec<Statement>> {
        let mut block = Vec::new();

        while self.current.kind != end {
            block.push(self.parse_statement()?);
        }

        Ok(block)
    }

    fn r#type(&mut self) -> ParserResult<Option<Type>> {
        if self.current.kind != TokenKind::Colon && self.current.kind != TokenKind::DoubleColon {
            Ok(None)
        } else {
            self.read();

            let r#type = self.identifier()?;
            
            if ! Type::valid(&r#type) {
                return Ok(None)
            }

            Ok(Some(Type::string(r#type)))
        }
    }

    pub fn parse(&mut self) -> ParserResult<Program> {
        let mut program = Vec::new();

        self.read();
        self.read();

        while self.current.kind != TokenKind::Eof {
            program.push(self.parse_statement()?);
        }

        Ok(program)
    }
}

fn is_prefix(kind: &TokenKind) -> bool {
    [TokenKind::Minus, TokenKind::Not].contains(kind)
}

fn prefix_binding_power(kind: &TokenKind) -> ((), u8) {
    match kind {
        TokenKind::Minus | TokenKind::Not => ((), 99),
        _ => unreachable!()
    }
}

fn prefix(kind: &TokenKind, rhs: Expression) -> Expression {
    Expression::Prefix(kind.into(), Box::new(rhs))
}

fn infix_binding_power(kind: &TokenKind) -> Option<(BindingPower, BindingPower)> {
    Some(match kind {
        TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent | TokenKind::DoubleAsterisk => (13, 14),
        TokenKind::Plus | TokenKind::Minus => (11, 12),
        TokenKind::GreaterThan | TokenKind::GreaterThanEquals | TokenKind::LessThan | TokenKind::LessThanEquals => (9, 10),
        TokenKind::EqualsEquals | TokenKind::NotEquals => (7, 8),
        TokenKind::And => (5, 6),
        TokenKind::Or => (3, 4),
        TokenKind::Equals | TokenKind::PlusEquals | TokenKind::MinusEquals | TokenKind::AsteriskEquals | TokenKind::SlashEquals => (2, 1),
        _ => return None
    })
}

fn infix(lhs: Expression, kind: &TokenKind, rhs: Expression) -> Expression {
    match kind {
        TokenKind::Equals => Expression::Assign(lhs.boxed(), rhs.boxed()),
        _ => Expression::Infix(lhs.boxed(), kind.into(), rhs.boxed())
    }
}

fn postfix_binding_power(kind: &TokenKind) -> Option<(BindingPower, ())> {
    Some(match kind {
        TokenKind::LeftParen | TokenKind::LeftBracket | TokenKind::Dot => (19, ()),
        _ => return None
    })
}

fn postfix(parser: &mut Parser, lhs: Expression, kind: &TokenKind) -> ParserResult<Expression> {
    match kind {
        TokenKind::LeftParen => {
            let args = parser.args()?;

            parser.read();

            Ok(Expression::Call(lhs.boxed(), args))
        },
        TokenKind::LeftBracket => {
            if parser.current.kind == TokenKind::RightBracket {
                parser.expect(TokenKind::RightBracket)?;

                return Ok(Expression::Index(lhs.boxed(), None))
            }

            let property = parser.expression(0)?;

            parser.expect(TokenKind::RightBracket)?;

            Ok(Expression::Index(lhs.boxed(), Some(property.boxed())))
        },
        TokenKind::Dot => {
            let path = parser.expression(19)?;

            Ok(Expression::Dot(lhs.boxed(), path.boxed()))
        },
        _ => todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Op;

    macro_rules! map {
        ($($key:expr => $value:expr),+) => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
        };
    }

    #[test]
    fn short_closures() {
        assert_eq!(parse(r##"
            (fn () -> "testing")
        "##), vec![
            Statement::Expression {
                expression: Expression::Closure(vec![], vec![
                    Statement::Return {
                        expression: Expression::String("testing".to_owned())
                    }
                ])
            }
        ]);
    }

    #[test]
    fn closures() {
        assert_eq!(parse(r##"
        (fn () {

        })
        "##), vec![
            Statement::Expression {
                expression: Expression::Closure(vec![], vec![])
            }
        ]);

        assert_eq!(parse(r##"
            (fn (name) {

            })
        "##), vec![
            Statement::Expression {
                expression: Expression::Closure(vec![
                    Parameter::new("name", None)
                ], vec![])
            }
        ]);

        assert_eq!(parse(r##"
            (fn (name, age) {

            })
        "##), vec![
            Statement::Expression {
                expression: Expression::Closure(vec![
                    Parameter::new("name", None),
                    Parameter::new("age", None),
                ], vec![])
            }
        ]);
    }

    #[test]
    fn uses() {
        assert_eq!(parse(r##"
            use File from "@std/fs"
            use File, Dir from "@std/fs"
        "##), vec![
            Statement::Use {
                module: String::from("@std/fs"),
                imports: vec![
                    String::from("File"),
                ]
            },
            Statement::Use {
                module: String::from("@std/fs"),
                imports: vec![
                    String::from("File"),
                    String::from("Dir"),
                ]
            }
        ]);
    }

    #[test]
    fn maps() {
        assert_eq!(parse(r##"
        {
            "foo": "bar"
        }
        "##), vec![
            Statement::Expression {
                expression: Expression::Map(map!{
                    String::from("foo") => Expression::String("bar".to_owned())
                })
            }
        ])
    }

    #[test]
    fn arrays() {
        assert_eq!(parse("[1, 2, 3,]"), vec![
            Statement::Expression {
                expression: Expression::Array(vec![
                    Expression::Number(1.0),
                    Expression::Number(2.0),
                    Expression::Number(3.0),
                ])
            }
        ]);

        assert_eq!(parse("[1][0]"), vec![
            Statement::Expression {
                expression: Expression::Index(
                    Expression::Array(vec![
                        Expression::Number(1.0),
                    ]).boxed(),
                    Some(Expression::Number(0.0).boxed())
                )
            }
        ]);
    }

    #[test]
    fn let_statements() {
        assert_eq!(parse("let name = 1"), vec![
            Statement::Let {
                identifier: String::from("name"),
                r#type: Some(Type::Number),
                initial: Expression::Number(1.0),
            },
        ]);

        assert_eq!(parse("let name: number = 1"), vec![
            Statement::Let {
                identifier: String::from("name"),
                r#type: Some(Type::Number),
                initial: Expression::Number(1.0),
            },
        ]);
    }

    #[test]
    fn fn_statements() {
        assert_eq!(parse("fn name() {}"), vec![
            Statement::Function {
                identifier: String::from("name"),
                parameters: Vec::new(),
                return_type: None,
                body: Vec::new(),
            }
        ]);

        assert_eq!(parse("fn name() :: number {}"), vec![
            Statement::Function {
                identifier: String::from("name"),
                parameters: Vec::new(),
                return_type: Some(Type::Number),
                body: Vec::new(),
            }
        ]);

        assert_eq!(parse("fn name(hello) {}"), vec![
            Statement::Function {
                identifier: String::from("name"),
                parameters: vec![
                    Parameter::new("hello", None),
                ],
                return_type: None,
                body: Vec::new(),
            }
        ]);

        assert_eq!(parse("fn name(hello: string) {}"), vec![
            Statement::Function {
                identifier: String::from("name"),
                parameters: vec![
                    Parameter::new("hello", Some(Type::String)),
                ],
                return_type: None,
                body: Vec::new(),
            }
        ]);

        assert_eq!(parse("fn name(hello: string) :: string {}"), vec![
            Statement::Function {
                identifier: String::from("name"),
                parameters: vec![
                    Parameter::new("hello", Some(Type::String)),
                ],
                return_type: Some(Type::String),
                body: Vec::new(),
            }
        ]);

        assert_eq!(parse(r##"
            fn name(hello: string) :: string {
                let name = "testing"
            }"##
        ), vec![
            Statement::Function {
                identifier: String::from("name"),
                parameters: vec![
                    Parameter::new("hello", Some(Type::String)),
                ],
                return_type: Some(Type::String),
                body: vec![
                    Statement::Let { identifier: String::from("name"), r#type: Some(Type::String), initial: Expression::String("testing".into()) },
                ],
            }
        ]);
    }

    #[test]
    fn returns() {
        assert_eq!(parse("return true"), vec![
            Statement::Return {
                expression: Expression::Bool(true),
            }
        ]);
    }

    #[test]
    fn if_statements() {
        assert_eq!(parse("if true {}"), vec![
            Statement::If {
                condition: Expression::Bool(true),
                then: vec![],
                otherwise: vec![],
            }
        ]);

        assert_eq!(parse("if true {} else {}"), vec![
            Statement::If {
                condition: Expression::Bool(true),
                then: vec![],
                otherwise: vec![],
            }
        ]);

        assert_eq!(parse("
            if true {
                let age = 1
            }
        "), vec![
            Statement::If {
                condition: Expression::Bool(true),
                then: vec![
                    Statement::Let { identifier: String::from("age"), r#type: Some(Type::Number), initial: Expression::Number(1.0) }
                ],
                otherwise: vec![],
            }
        ]);

        assert_eq!(parse("
            if true {

            } else {
                let age = 1
            }
        "), vec![
            Statement::If {
                condition: Expression::Bool(true),
                then: vec![],
                otherwise: vec![
                    Statement::Let { identifier: String::from("age"), r#type: Some(Type::Number), initial: Expression::Number(1.0) }
                ],
            }
        ]);
    }

    #[test]
    fn while_statements() {
        assert_eq!(parse("while true {}"), vec![
            Statement::While { condition: Expression::Bool(true), then: vec![] }
        ]);

        assert_eq!(parse("while true { 1 }"), vec![
            Statement::While { condition: Expression::Bool(true), then: vec![
                Statement::Expression { expression: Expression::Number(1.0) },
            ] }
        ]);

        assert_eq!(parse("
            while true {
                break
            }
        "), vec![
            Statement::While { condition: Expression::Bool(true), then: vec![
                Statement::Break,
            ] }
        ]);

        assert_eq!(parse("
            while true {
                continue
            }`
        "), vec![
            Statement::While { condition: Expression::Bool(true), then: vec![
                Statement::Continue,
            ] }
        ])
    }

    #[test]
    fn prefixes() {
        assert_eq!(parse("-1"), vec![
            Statement::Expression { expression: Expression::Prefix(Op::Subtract, Box::new(Expression::Number(1.0))) },
        ]);
    }

    #[test]
    fn infixes() {
        assert_eq!(parse("1 + 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::Add, Expression::Number(1.0).boxed()) }
        ]);

        assert_eq!(parse("1 - 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::Subtract, Expression::Number(1.0).boxed()) }
        ]);

        assert_eq!(parse("1 * 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::Multiply, Expression::Number(1.0).boxed()) }
        ]);

        assert_eq!(parse("1 / 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::Divide, Expression::Number(1.0).boxed()) }
        ]);

        assert_eq!(parse("1 + 2 * 3"), vec![
            Statement::Expression {
                expression: Expression::Infix(
                    Expression::Number(1.0).boxed(), Op::Add, Expression::Infix(
                        Expression::Number(2.0).boxed(),
                        Op::Multiply,
                        Expression::Number(3.0).boxed()
                    ).boxed()
                )
            }
        ]);

        assert_eq!(parse("1 + 2 * 3 / 4"), vec![
            Statement::Expression {
                expression: Expression::Infix(
                    Expression::Number(1.0).boxed(), Op::Add, Expression::Infix(
                        Expression::Infix(
                            Expression::Number(2.0).boxed(),
                            Op::Multiply,
                            Expression::Number(3.0).boxed()
                        ).boxed(),
                        Op::Divide,
                        Expression::Number(4.0).boxed()
                    ).boxed()
                )
            }
        ]);

        assert_eq!(parse("1 > 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::GreaterThan, Expression::Number(1.0).boxed()) }
        ]);

        assert_eq!(parse("1 < 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::LessThan, Expression::Number(1.0).boxed()) }
        ]);

        assert_eq!(parse("1 >= 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::GreaterThanEquals, Expression::Number(1.0).boxed()) }
        ]);

        assert_eq!(parse("1 <= 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::LessThanEquals, Expression::Number(1.0).boxed()) }
        ]);

        assert_eq!(parse("1 == 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::Equals, Expression::Number(1.0).boxed()) }
        ]);

        assert_eq!(parse("1 != 1"), vec![
            Statement::Expression { expression: Expression::Infix(Expression::Number(1.0).boxed(), Op::NotEquals, Expression::Number(1.0).boxed()) }
        ]);
        
        assert_eq!(parse("foo = 2"), vec![
            Statement::Expression {
                expression: Expression::Assign(
                    Expression::Identifier("foo".to_owned()).boxed(),
                    Expression::Number(2.0).boxed()
                )
            }
        ]);
    }

    #[test]
    fn postfixes() {
        assert_eq!(parse("foo()"), vec![
            Statement::Expression {
                expression: Expression::Call(
                    Expression::Identifier("foo".to_owned()).boxed(),
                    vec![]
                )
            }
        ]);

        assert_eq!(parse("foo() + foo()"), vec![
            Statement::Expression {
                expression: Expression::Infix(
                    Expression::Call(Expression::Identifier("foo".to_owned()).boxed(), Vec::new()).boxed(),
                    Op::Add,
                    Expression::Call(Expression::Identifier("foo".to_owned()).boxed(), Vec::new()).boxed(),
                )
            }
        ]);

        assert_eq!(parse("foo(1)"), vec![
            Statement::Expression {
                expression: Expression::Call(
                    Expression::Identifier("foo".to_owned()).boxed(),
                    vec![
                        Expression::Number(1.0),
                    ]
                )
            }
        ]);

        assert_eq!(parse("foo(1, 2, 3)"), vec![
            Statement::Expression {
                expression: Expression::Call(
                    Expression::Identifier("foo".to_owned()).boxed(),
                    vec![
                        Expression::Number(1.0),
                        Expression::Number(2.0),
                        Expression::Number(3.0),
                    ]
                )
            }
        ]);

        assert_eq!(parse("foo(1, 2, 3,)"), vec![
            Statement::Expression {
                expression: Expression::Call(
                    Expression::Identifier("foo".to_owned()).boxed(),
                    vec![
                        Expression::Number(1.0),
                        Expression::Number(2.0),
                        Expression::Number(3.0),
                    ]
                )
            }
        ]);
    }

    fn parse(source: &str) -> Program {
        let lexer = Lexer::new(source);

        Parser::new(lexer).parse().unwrap()
    }
}