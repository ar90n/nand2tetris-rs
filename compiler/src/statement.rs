use anyhow::*;

use super::expression::*;
use super::foundation::*;
use super::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement(pub Box<Optional<Expression>>);

impl Parsable for ReturnStatement {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (_, rem) = token_parser(Token::Return)(tokens)?;
        let (e, rem) = Optional::<Expression>::parse(rem)?;
        let (_, rem) = token_parser(Token::Semicolon)(rem)?;
        Ok((Box::new(Self(e)), rem))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoStatement(pub Box<SubroutineCall>);

impl Parsable for DoStatement {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (_, rem) = token_parser(Token::Do)(tokens)?;
        let (s, rem) = SubroutineCall::parse(rem)?;
        let (_, rem) = token_parser(Token::Semicolon)(rem)?;
        Ok((Box::new(Self(s)), rem))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStatement {
    pub condition: Box<Expression>,
    pub body: Box<Statements>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub then_body: Box<Statements>,
    pub else_body: Box<Option<Box<Statements>>>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStatement {
    pub var_name: Box<Identifier>,
    pub array_size: Box<Option<Box<Expression>>>,
    pub expression: Box<Expression>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statements {
    pub statements: Box<Collection<Statement>>,
}

impl Parsable for Statements {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (statements, rem) = Collection::<Statement>::parse(tokens)?;
        Ok((Box::new(Self { statements }), rem))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentence() {
        assert_eq!(
            ReturnStatement::parse(
                &[Token::Return, Token::IntegerConstant(123), Token::Semicolon,]
            )
            .unwrap(),
            (
                Box::new(ReturnStatement(Box::new(Optional::new(Some(Box::new(
                    Expression {
                        term: Box::new(Term::Constant(Box::new(Constant::Integer(123i16)))),
                        extras: Box::new(Collection::<Seq2<Op, Term>>::new(vec![]))
                    }
                )))))),
                [].as_slice()
            )
        );

        assert_eq!(
            IfStatement::parse(&[
                Token::If,
                Token::LParen,
                Token::IntegerConstant(123),
                Token::RParen,
                Token::LBrace,
                Token::Return,
                Token::IntegerConstant(321),
                Token::Semicolon,
                Token::RBrace,
                Token::Else,
                Token::LBrace,
                Token::Return,
                Token::IntegerConstant(111),
                Token::Semicolon,
                Token::RBrace,
            ])
            .unwrap(),
            (
                Box::new(IfStatement {
                    condition: Box::new(Expression {
                        term: Box::new(Term::Constant(Box::new(Constant::Integer(123i16)))),
                        extras: Box::new(Collection::<Seq2<Op, Term>>::new(vec![]))
                    }),
                    then_body: Box::new(Statements {
                        statements: Box::new(Collection::<Statement>::new(vec![Box::new(
                            Statement::Return(Box::new(ReturnStatement(Box::new(Optional::new(
                                Some(Box::new(Expression {
                                    term: Box::new(Term::Constant(Box::new(Constant::Integer(
                                        321i16
                                    )))),
                                    extras: Box::new(Collection::<Seq2<Op, Term>>::new(vec![]))
                                }))
                            )))))
                        )]))
                    }),
                    else_body: Box::new(Some(Box::new(Statements {
                        statements: Box::new(Collection::<Statement>::new(vec![Box::new(
                            Statement::Return(Box::new(ReturnStatement(Box::new(Optional::new(
                                Some(Box::new(Expression {
                                    term: Box::new(Term::Constant(Box::new(Constant::Integer(
                                        111i16
                                    )))),
                                    extras: Box::new(Collection::<Seq2<Op, Term>>::new(vec![]))
                                }))
                            )))))
                        )]))
                    })))
                }),
                [].as_slice()
            )
        );
    }
}
