use anyhow::*;

use super::foundation::*;
use super::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionList {
    pub expressions: Box<Collection<Expression>>,
}

impl ExpressionList {
    pub fn len(&self) -> usize {
        self.expressions.len()
    }
}

impl Parsable for ExpressionList {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (vs, rem) = option_parser(seq2_parser(
            Expression::parse,
            repeat_parser(seq2_parser(token_parser(Token::Comma), Expression::parse)),
        ))(tokens)?;

        let expressions = if let Some(es) = *vs {
            let (head, tails) = *es;
            let mut expressions = vec![head];
            expressions.extend(tails.into_iter().map(|p| p.1).collect::<Vec<_>>());
            Collection::new(expressions)
        } else {
            Collection::default()
        };
        let expressions = Box::new(expressions);
        Ok((Box::new(Self { expressions }), rem))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubroutineCall {
    ExternalCall(Box<Identifier>, Box<Identifier>, Box<ExpressionList>),
    InternalCall(Box<Identifier>, Box<ExpressionList>),
}
impl SubroutineCall {
    pub fn parse_method_call(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (var_name, rem) = Identifier::parse(tokens)?;
        let (_, rem) = token_parser(Token::Period)(rem)?;
        let (method_name, rem) = Identifier::parse(rem)?;
        let (args, rem) =
            surround_parser(ExpressionList::parse, Token::LParen, Token::RParen)(rem)?;
        Ok((Box::new(Self::ExternalCall(var_name, method_name, args)), rem))
    }

    pub fn parse_function_call(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (function_name, rem) = Identifier::parse(tokens)?;
        let (args, rem) =
            surround_parser(ExpressionList::parse, Token::LParen, Token::RParen)(rem)?;
        Ok((Box::new(Self::InternalCall(function_name, args)), rem))
    }
}

impl Parsable for SubroutineCall {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        Self::parse_method_call(tokens).or_else(|_| Self::parse_function_call(tokens))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Constant(Box<Constant>),
    Variable(Box<Identifier>),
    ArrayAccess(Box<Identifier>, Box<Expression>),
    SubroutineCall(Box<SubroutineCall>),
    Expression(Box<Expression>),
    UnaryOp(Box<UnaryOp>, Box<Term>),
}

impl Term {
    fn parse_constant(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (c, rem) = Constant::parse(tokens)?;
        Ok((Box::new(Self::Constant(c)), rem))
    }

    fn parse_variable(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (v, rem) = Identifier::parse(tokens)?;
        Ok((Box::new(Self::Variable(v)), rem))
    }

    fn parse_array_access(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (var_name, rem) = Identifier::parse(tokens)?;
        let (index, rem) =
            surround_parser(Expression::parse, Token::LBracket, Token::RBracket)(rem)?;

        Ok((Box::new(Self::ArrayAccess(var_name, index)), rem))
    }

    fn parse_subroutine_call(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (s, rem) = SubroutineCall::parse(tokens)?;
        Ok((Box::new(Self::SubroutineCall(s)), rem))
    }

    fn parse_expression(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (e, rem) = surround_parser(Expression::parse, Token::LParen, Token::RParen)(tokens)?;

        Ok((Box::new(Self::Expression(e)), rem))
    }

    fn parse_unaryop(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (op, rem) = UnaryOp::parse(tokens)?;
        let (t, rem) = Term::parse(rem)?;
        Ok((Box::new(Self::UnaryOp(op, t)), rem))
    }
}

impl Parsable for Term {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        Self::parse_constant(tokens)
            .or_else(|_| Self::parse_subroutine_call(tokens))
            .or_else(|_| Self::parse_constant(tokens))
            .or_else(|_| Self::parse_array_access(tokens))
            .or_else(|_| Self::parse_variable(tokens))
            .or_else(|_| Self::parse_expression(tokens))
            .or_else(|_| Self::parse_unaryop(tokens))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub term: Box<Term>,
    pub extras: Box<Collection<Seq2<Op, Term>>>,
}

impl Parsable for Expression {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (term, rem) = Term::parse(tokens)?;
        let (extras, rem) = Collection::<Seq2<Op, Term>>::parse(rem)?;
        Ok((Box::new(Self { term, extras }), rem))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[test]
    fn test_expression_list() {
        assert_eq!(
            ExpressionList::parse(&[
                Token::IntegerConstant(123),
                Token::Comma,
                Token::IntegerConstant(345)
            ])
            .unwrap(),
            (
                Box::new(ExpressionList {
                    expressions: Box::new(Collection::<Expression>::new(vec![
                        Box::new(Expression {
                            term: Box::new(Term::Constant(Box::new(Constant::Integer(123i16)))),
                            extras: Box::new(Collection::<Seq2<Op, Term>>::new(vec![]))
                        }),
                        Box::new(Expression {
                            term: Box::new(Term::Constant(Box::new(Constant::Integer(345i16)))),
                            extras: Box::new(Collection::<Seq2<Op, Term>>::new(vec![]))
                        })
                    ]))
                }),
                [].as_slice()
            )
        );
    }
}
