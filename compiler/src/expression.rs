use anyhow::*;

use super::foundation::*;
use super::parsable::*;
use super::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionList {
    pub expressions: Box<Collection<Expression>>,
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

impl DumpXml for ExpressionList {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("expressionList", level);
        let mut tags = vec![open_tag];

        for (i, expression) in self.expressions.items.iter().enumerate() {
            if 0 < i {
                tags.push(Token::Comma.dump_as_xml(level + 1));
            }
            tags.push(expression.dump_as_xml(level + 1));
        }

        tags.push(close_tag);
        tags.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubroutineCall {
    MethodCall(Box<Identifier>, Box<Identifier>, Box<ExpressionList>),
    FunctionCall(Box<Identifier>, Box<ExpressionList>),
}
impl SubroutineCall {
    pub fn parse_method_call(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (var_name, rem) = Identifier::parse(tokens)?;
        let (_, rem) = token_parser(Token::Period)(rem)?;
        let (method_name, rem) = Identifier::parse(rem)?;
        let (args, rem) =
            surround_parser(ExpressionList::parse, Token::LParen, Token::RParen)(rem)?;
        Ok((Box::new(Self::MethodCall(var_name, method_name, args)), rem))
    }

    pub fn parse_function_call(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (function_name, rem) = Identifier::parse(tokens)?;
        let (args, rem) =
            surround_parser(ExpressionList::parse, Token::LParen, Token::RParen)(rem)?;
        Ok((Box::new(Self::FunctionCall(function_name, args)), rem))
    }
}

impl Parsable for SubroutineCall {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        Self::parse_method_call(tokens).or_else(|_| Self::parse_function_call(tokens))
    }
}

impl DumpXml for SubroutineCall {
    fn dump_as_xml(&self, level: usize) -> String {
        match self {
            Self::MethodCall(var_name, method_name, args) => {
                let lines = vec![
                    var_name.dump_as_xml(level),
                    Token::Period.dump_as_xml(level),
                    method_name.dump_as_xml(level),
                    Token::LParen.dump_as_xml(level),
                    args.dump_as_xml(level),
                    Token::RParen.dump_as_xml(level),
                ];
                lines.join("\n")
            }
            Self::FunctionCall(function_name, args) => {
                let lines = vec![
                    function_name.dump_as_xml(level),
                    Token::LParen.dump_as_xml(level),
                    args.dump_as_xml(level),
                    Token::RParen.dump_as_xml(level),
                ];
                lines.join("\n")
            }
        }
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

impl DumpXml for Term {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("term", level);
        let mut tags = vec![open_tag];
        let body = match self {
            Self::Constant(c) => vec![c.dump_as_xml(level + 1)],
            Self::Variable(v) => vec![v.dump_as_xml(level + 1)],
            Self::ArrayAccess(v, e) => vec![
                v.dump_as_xml(level + 1),
                Token::LBracket.dump_as_xml(level + 1),
                e.dump_as_xml(level + 1),
                Token::RBracket.dump_as_xml(level + 1),
            ],
            Self::SubroutineCall(s) => vec![s.dump_as_xml(level + 1)],
            Self::Expression(e) => vec![
                Token::LParen.dump_as_xml(level + 1),
                e.dump_as_xml(level + 1),
                Token::RParen.dump_as_xml(level + 1),
            ],
            Self::UnaryOp(op, t) => vec![op.dump_as_xml(level + 1), t.dump_as_xml(level + 1)],
        };
        tags.extend(body);
        tags.push(close_tag);
        tags.join("\n")
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

impl DumpXml for Expression {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("expression", level);

        let mut tags = vec![open_tag, self.term.dump_as_xml(level + 1)];

        self.extras.items.iter().for_each(|pair| {
            let (ref op, ref term) = **pair;
            tags.push(op.dump_as_xml(level + 1));
            tags.push(term.dump_as_xml(level + 1));
        });

        tags.push(close_tag);
        tags.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //    fn test_expression_list() {
    //        assert_eq!(
    //            ExpressionList::parse(&[
    //                Token::Identifier("a".to_string()),
    //                Token::LParen,
    //                Token::IntegerConstant(1),
    //                Token::RParen,
    //                Token::Semicolon
    //            ])
    //            .unwrap()
    //            .0
    //            .dump_as_xml(),
    //            ("a".to_string())
    //        );
    //    }

    //    #[test]
    //    fn test_expression_list() {
    //        assert_eq!(
    //            ExpressionList::parse(&[Token::IntegerConstant(123)]).unwrap(),
    //            (
    //                Box::new(ExpressionList {
    //                    expressions: vec![Box::new(Expression {
    //                        term: Box::new(Term::Constant(123)),
    //                        extras: Box::new(vec![])
    //                    })]
    //                }),
    //                [].as_slice()
    //            )
    //        );
    //        //assert_eq!(
    //        //    ExpressionList::parse("1,2,3 4").unwrap(),
    //        //    (
    //        //        Box::new(ExpressionList {
    //        //            expressions: vec![
    //        //                Box::new(Expression {
    //        //                    term: Box::new(Term::Constant(1)),
    //        //                    extras: None
    //        //                }),
    //        //                Box::new(Expression {
    //        //                    term: Box::new(Term::Constant(2)),
    //        //                    extras: None
    //        //                }),
    //        //                Box::new(Expression {
    //        //                    term: Box::new(Term::Constant(3)),
    //        //                    extras: None
    //        //                })
    //        //            ]
    //        //        }),
    //        //        " 4"
    //        //    )
    //        //);
    //        //assert!(ExpressionList::parse(");").is_err());
    //    }
    //
    #[test]
    fn test_term() {
        assert_eq!(
            Term::parse(&[Token::IntegerConstant(123)]).unwrap(),
            (
                Box::new(Term::Constant(Box::new(Constant::Integer(123)))),
                [].as_slice()
            )
        );
        //        assert_eq!(
        //            Term::parse(&[Token::StringConstant("hello".to_string())]).unwrap(),
        //            (Box::new(Term::Constant("hello".to_string())), [].as_slice())
        //        );
        //        assert_eq!(
        //            Term::parse(&[Token::True]).unwrap(),
        //            (
        //                Box::new(Term::Constant(KeywordConstant::True)),
        //                [].as_slice()
        //            )
        //        );
        //        assert_eq!(
        //            Term::parse(&[Token::False]).unwrap(),
        //            (
        //                Box::new(Term::Constant(KeywordConstant::False)),
        //                [].as_slice()
        //            )
        //        );
        //        assert_eq!(
        //            Term::parse(&[Token::Null]).unwrap(),
        //            (
        //                Box::new(Term::Constant(KeywordConstant::Null)),
        //                [].as_slice()
        //            )
        //        );
        //        assert_eq!(
        //            Term::parse(&[Token::This]).unwrap(),
        //            (
        //                Box::new(Term::Constant(KeywordConstant::This)),
        //                [].as_slice()
        //            )
        //        );
        //        assert_eq!(
        //            Term::parse(&[Token::Identifier("abc".to_string())]).unwrap(),
        //            (
        //                Box::new(Term::Variable(Box::new(Identifier("abc".to_string())))),
        //                [].as_slice()
        //            )
        //        );
        //
        //        assert_eq!(
        //            Term::parse(&[
        //                Token::Identifier("abc".to_string()),
        //                Token::LBracket,
        //                Token::IntegerConstant(123),
        //                Token::Plus,
        //                Token::IntegerConstant(321),
        //                Token::RBracket
        //            ])
        //            .unwrap(),
        //            (
        //                Box::new(Term::ArrayAccess(
        //                    Box::new(Identifier("abc".to_string())),
        //                    Box::new(Expression {
        //                        term: Box::new(Term::Constant(123)),
        //                        extras: Box::new(vec![Box::new((
        //                            Box::new(Op::Plus),
        //                            Box::new(Term::Constant(321))
        //                        ))])
        //                    })
        //                )),
        //                [].as_slice()
        //            )
        //        );
        //
        //        assert_eq!(
        //            Term::parse(&[
        //                Token::Identifier("obj".to_string()),
        //                Token::Period,
        //                Token::Identifier("method".to_string()),
        //                Token::LParen,
        //                Token::IntegerConstant(321),
        //                Token::RParen
        //            ])
        //            .unwrap(),
        //            (
        //                Box::new(Term::SubroutineCall(Box::new(SubroutineCall::MethodCall(
        //                    Box::new(Identifier("obj".to_string())),
        //                    Box::new(Identifier("method".to_string())),
        //                    Box::new(ExpressionList {
        //                        expressions: vec![Box::new(Expression {
        //                            term: Box::new(Term::Constant(321)),
        //                            extras: Box::new(vec![])
        //                        })]
        //                    })
        //                )))),
        //                [].as_slice()
        //            )
        //        );
        //
        //        assert_eq!(
        //            Term::parse(&[Token::LParen, Token::IntegerConstant(321), Token::RParen]).unwrap(),
        //            (
        //                Box::new(Term::Expression(Box::new(Expression {
        //                    term: Box::new(Term::Constant(321)),
        //                    extras: Box::new(vec![])
        //                }))),
        //                [].as_slice()
        //            )
        //        );
        //
        //        assert_eq!(
        //            Term::parse(&[Token::Minus, Token::IntegerConstant(321)]).unwrap(),
        //            (
        //                Box::new(Term::UnaryOp(
        //                    Box::new(UnaryOp::Minus),
        //                    Box::new(Term::Constant(321)),
        //                )),
        //                [].as_slice()
        //            )
        //        );
        //
        //        //        assert_eq!(
        //        //            Term::parse(&[Token::Identifier("hello")]).unwrap(),
        //        //            (Box::new(Term::Identifier("hello")), [].as_slice())
        //        //        );
        //        //        assert_eq!(
        //        //            Term::parse(&[Token::IntegerConstant(123), Token::Asterisk]).unwrap(),
        //        //            (
        //        //                Box::new(Term::TermOp {
        //        //                    term: Box::new(Term::Constant(123)),
        //        //                    op: Box::new(Op::Asterisk),
        //        //                }),
        //        //                [Token::Asterisk].as_slice()
        //        //            )
        //        //        );
    }
}
