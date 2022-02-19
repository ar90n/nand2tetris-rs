use anyhow::*;

use super::expression::*;
use super::foundation::*;
use super::parsable::*;
use super::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement(Box<Option<Box<Expression>>>);

impl Parsable for ReturnStatement {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (_, rem) = token_parser(Token::Return)(tokens)?;
        let (e, rem) = option_parser(Expression::parse)(rem)?;
        let (_, rem) = token_parser(Token::Semicolon)(rem)?;
        Ok((Box::new(Self(e)), rem))
    }
}

impl DumpXml for ReturnStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("returnStatement", level);
        let mut tags = vec![open_tag, Token::Return.dump_as_xml(level + 1)];

        if let Some(ref v) = *self.0 {
            tags.push(v.dump_as_xml(level + 1));
        }
        tags.push(Token::Semicolon.dump_as_xml(level + 1));
        tags.push(close_tag);
        tags.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoStatement(Box<SubroutineCall>);

impl Parsable for DoStatement {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (_, rem) = token_parser(Token::Do)(tokens)?;
        let (s, rem) = SubroutineCall::parse(rem)?;
        let (_, rem) = token_parser(Token::Semicolon)(rem)?;
        Ok((Box::new(Self(s)), rem))
    }
}

impl DumpXml for DoStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("doStatement", level);
        vec![
            open_tag,
            Token::Do.dump_as_xml(level + 1),
            self.0.dump_as_xml(level + 1),
            Token::Semicolon.dump_as_xml(level + 1),
            close_tag,
        ]
        .join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStatement {
    condition: Box<Expression>,
    body: Box<Statements>,
}

impl Parsable for WhileStatement {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (_, rem) = token_parser(Token::While)(tokens)?;

        let (condition, rem) =
            surround_parser(Expression::parse, Token::LParen, Token::RParen)(rem)?;
        let (body, rem) = surround_parser(Statements::parse, Token::LBrace, Token::RBrace)(rem)?;

        Ok((Box::new(Self { condition, body }), rem))
    }
}

impl DumpXml for WhileStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("whileStatement", level);
        vec![
            open_tag,
            Token::While.dump_as_xml(level + 1),
            Token::LParen.dump_as_xml(level + 1),
            self.condition.dump_as_xml(level + 1),
            Token::RParen.dump_as_xml(level + 1),
            Token::LBrace.dump_as_xml(level + 1),
            self.body.dump_as_xml(level + 1),
            Token::RBrace.dump_as_xml(level + 1),
            close_tag,
        ]
        .join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStatement {
    condition: Box<Expression>,
    then_body: Box<Statements>,
    else_body: Box<Option<Box<Statements>>>,
}

impl Parsable for IfStatement {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (_, rem) = token_parser(Token::If)(tokens)?;

        let (condition, rem) =
            surround_parser(Expression::parse, Token::LParen, Token::RParen)(rem)?;
        let (then_body, rem) =
            surround_parser(Statements::parse, Token::LBrace, Token::RBrace)(rem)?;
        let (else_body, rem) = option_parser(drop1_parser(
            token_parser(Token::Else),
            surround_parser(Statements::parse, Token::LBrace, Token::RBrace),
        ))(rem)?;
        Ok((
            Box::new(Self {
                condition,
                then_body,
                else_body,
            }),
            rem,
        ))
    }
}

impl DumpXml for IfStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("ifStatement", level);
        let mut tags = vec![
            open_tag,
            Token::If.dump_as_xml(level + 1),
            Token::LParen.dump_as_xml(level + 1),
            self.condition.dump_as_xml(level + 1),
            Token::RParen.dump_as_xml(level + 1),
            Token::LBrace.dump_as_xml(level + 1),
            self.then_body.dump_as_xml(level + 1),
            Token::RBrace.dump_as_xml(level + 1),
        ];
        if let Some(ref v) = *self.else_body {
            tags.push(Token::Else.dump_as_xml(level + 1));
            tags.push(Token::LBrace.dump_as_xml(level + 1));
            tags.push(v.dump_as_xml(level + 1));
            tags.push(Token::RBrace.dump_as_xml(level + 1));
        }
        tags.push(close_tag);
        tags.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStatement {
    var_name: Box<Identifier>,
    array_size: Box<Option<Box<Expression>>>,
    expression: Box<Expression>,
}

impl Parsable for LetStatement {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (_, rem) = token_parser(Token::Let)(tokens)?;
        let (var_name, rem) = Identifier::parse(rem)?;
        let (array_size, rem) = option_parser(surround_parser(
            Expression::parse,
            Token::LBracket,
            Token::RBracket,
        ))(rem)?;
        let (_, rem) = token_parser(Token::Eq)(rem)?;
        let (expression, rem) = Expression::parse(rem)?;
        let (_, rem) = token_parser(Token::Semicolon)(rem)?;

        Ok((
            Box::new(Self {
                var_name,
                array_size,
                expression,
            }),
            rem,
        ))
    }
}

impl DumpXml for LetStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("letStatement", level);
        let mut tags = vec![
            open_tag,
            Token::Let.dump_as_xml(level + 1),
            self.var_name.dump_as_xml(level + 1),
        ];

        if let Some(ref v) = *self.array_size {
            tags.extend([
                Token::LBracket.dump_as_xml(level + 1),
                v.dump_as_xml(level + 1),
                Token::RBracket.dump_as_xml(level + 1),
            ]);
        }

        tags.extend([
            Token::Eq.dump_as_xml(level + 1),
            self.expression.dump_as_xml(level + 1),
            Token::Semicolon.dump_as_xml(level + 1),
            close_tag,
        ]);
        tags.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let(Box<LetStatement>),
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    Do(Box<DoStatement>),
    Return(Box<ReturnStatement>),
}
impl Statement {
    pub fn parse_let(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (s, rem) = LetStatement::parse(tokens)?;
        Ok((Box::new(Self::Let(s)), rem))
    }
    pub fn parse_if(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (s, rem) = IfStatement::parse(tokens)?;
        Ok((Box::new(Self::If(s)), rem))
    }
    pub fn parse_while(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (s, rem) = WhileStatement::parse(tokens)?;
        Ok((Box::new(Self::While(s)), rem))
    }
    pub fn parse_do(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (s, rem) = DoStatement::parse(tokens)?;
        Ok((Box::new(Self::Do(s)), rem))
    }
    pub fn parse_return(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (s, rem) = ReturnStatement::parse(tokens)?;
        Ok((Box::new(Self::Return(s)), rem))
    }
}

impl Parsable for Statement {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        Self::parse_let(tokens)
            .or_else(|_| Self::parse_if(tokens))
            .or_else(|_| Self::parse_while(tokens))
            .or_else(|_| Self::parse_do(tokens))
            .or_else(|_| Self::parse_return(tokens))
    }
}

impl DumpXml for Statement {
    fn dump_as_xml(&self, level: usize) -> String {
        match self {
            Self::Let(v) => v.dump_as_xml(level),
            Self::If(v) => v.dump_as_xml(level),
            Self::While(v) => v.dump_as_xml(level),
            Self::Do(v) => v.dump_as_xml(level),
            Self::Return(v) => v.dump_as_xml(level),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statements {
    pub statements: Box<Vec<Box<Statement>>>,
}

impl Parsable for Statements {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (statements, rem) = repeat_parser(Statement::parse)(tokens)?;
        Ok((Box::new(Self { statements }), rem))
    }
}

impl DumpXml for Statements {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("statements", level);
        let mut tags = vec![open_tag];

        self.statements.iter().for_each(|s| {
            tags.push(s.dump_as_xml(level + 1));
        });

        tags.push(close_tag);
        tags.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //    #[test]
    //    fn test_sentence() {
    //        assert_eq!(
    //            Statements::parse(&[
    //                Token::Let,
    //                Token::Identifier("x".to_string()),
    //                Token::Eq,
    //                Token::Identifier("sum".to_ascii_lowercase()),
    //                Token::LParen,
    //                Token::Identifier("p".to_ascii_lowercase()),
    //                Token::LParen,
    //                Token::StringConstant("foo".to_string()),
    //                Token::RParen,
    //                Token::RParen,
    //                Token::Semicolon,
    //                Token::If,
    //                Token::LParen,
    //                Token::True,
    //                Token::RParen,
    //                Token::LBrace,
    //                Token::Return,
    //                Token::IntegerConstant(20),
    //                Token::Semicolon,
    //                Token::RBrace,
    //                Token::Else,
    //                Token::LBrace,
    //                Token::Return,
    //                Token::IntegerConstant(100),
    //                Token::Semicolon,
    //                Token::RBrace,
    //            ])
    //            .unwrap()
    //            .0
    //            .dump_as_xml()
    //            ,
    //            ("a".to_string())
    //        );
    //    }
    //
    //    #[test]
    //    fn test_sentence() {
    //        assert_eq!(
    //            ReturnStatement::parse(
    //                &[Token::Return, Token::IntegerConstant(123), Token::Semicolon,]
    //            )
    //            .unwrap(),
    //            (
    //                Box::new(ReturnStatement(Box::new(Some(Box::new(Expression {
    //                    term: Box::new(Term::IntegerConstant(123)),
    //                    extras: Box::new(vec![])
    //                }))))),
    //                [].as_slice()
    //            )
    //        );
    //
    //        assert_eq!(
    //            Statement::parse(&[Token::Return, Token::IntegerConstant(123), Token::Semicolon])
    //                .unwrap(),
    //            (
    //                Box::new(Statement::Return(Box::new(ReturnStatement(Box::new(
    //                    Some(Box::new(Expression {
    //                        term: Box::new(Term::IntegerConstant(123)),
    //                        extras: Box::new(vec![])
    //                    }))
    //                ))))),
    //                [].as_slice()
    //            )
    //        );
    //        assert_eq!(
    //            Statements::parse(&[Token::Return, Token::IntegerConstant(123), Token::Semicolon])
    //                .unwrap(),
    //            (
    //                Box::new(Statements {
    //                    statements: Box::new(vec![Box::new(Statement::Return(Box::new(
    //                        ReturnStatement(Box::new(Some(Box::new(Expression {
    //                            term: Box::new(Term::IntegerConstant(123)),
    //                            extras: Box::new(vec![])
    //                        }))))
    //                    )))])
    //                }),
    //                [].as_slice()
    //            )
    //        );
    //
    //        assert_eq!(
    //            IfStatement::parse(&[
    //                Token::If,
    //                Token::LParen,
    //                Token::IntegerConstant(123),
    //                Token::RParen,
    //                Token::LBrace,
    //                Token::Return,
    //                Token::IntegerConstant(321),
    //                Token::Semicolon,
    //                Token::RBrace
    //            ])
    //            .unwrap(),
    //            (
    //                Box::new(IfStatement {
    //                    condition: Box::new(Expression {
    //                        term: Box::new(Term::IntegerConstant(123)),
    //                        extras: Box::new(vec![])
    //                    }),
    //                    then_body: Box::new(Statements {
    //                        statements: Box::new(vec![Box::new(Statement::Return(Box::new(
    //                            ReturnStatement(Box::new(Some(Box::new(Expression {
    //                                term: Box::new(Term::IntegerConstant(321)),
    //                                extras: Box::new(vec![])
    //                            }))))
    //                        ))),])
    //                    }),
    //                    else_body: Box::new(None)
    //                }),
    //                [].as_slice()
    //            )
    //        );
    //    }
}
