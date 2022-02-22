use anyhow::*;

use super::foundation::*;
use super::statement::*;
use super::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarDec {
    pub type_: Box<Type>,
    pub var_name: Box<Identifier>,
    pub extra_var_names: Box<Vec<Box<Identifier>>>,
}

impl Parsable for VarDec {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (_, rem) = token_parser(Token::Var)(tokens)?;
        let (type_, rem) = Type::parse(rem)?;
        let (var_name, rem) = Identifier::parse(rem)?;
        let (extra_var_names, rem) =
            repeat_parser(drop1_parser(token_parser(Token::Comma), Identifier::parse))(rem)?;
        let (_, rem) = token_parser(Token::Semicolon)(rem)?;
        Ok((
            Box::new(Self {
                type_,
                var_name,
                extra_var_names,
            }),
            rem,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubroutineBody {
    pub var_decs: Box<Vec<Box<VarDec>>>,
    pub statements: Box<Statements>,
}

impl Parsable for SubroutineBody {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (ret, rem) = surround_parser(
            seq2_parser(repeat_parser(VarDec::parse), Statements::parse),
            Token::LBrace,
            Token::RBrace,
        )(tokens)?;
        Ok((
            Box::new(Self {
                var_decs: ret.0,
                statements: ret.1,
            }),
            rem,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterList {
    pub params: Vec<Box<(Box<Type>, Box<Identifier>)>>,
}

impl Parsable for ParameterList {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (vs, rem) = option_parser(seq2_parser(
            seq2_parser(Type::parse, Identifier::parse),
            repeat_parser(drop1_parser(
                token_parser(Token::Comma),
                seq2_parser(Type::parse, Identifier::parse),
            )),
        ))(tokens)?;

        let mut params = vec![];
        if let Some(es) = *vs {
            let (head, tails) = *es;
            params.push(head);
            for vs in *tails {
                params.push(vs);
            }
        }
        Ok((Box::new(Self { params }), rem))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubroutineDec {
    pub kind: Box<SubroutineKind>,
    pub type_: Box<SubroutineType>,
    pub subroutine_name: Box<Identifier>,
    pub params: Box<ParameterList>,
    pub body: Box<SubroutineBody>,
}

impl Parsable for SubroutineDec {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (kind, rem) = SubroutineKind::parse(tokens)?;
        let (type_, rem) = SubroutineType::parse(rem)?;
        let (subroutine_name, rem) = Identifier::parse(rem)?;
        let (params, rem) =
            surround_parser(ParameterList::parse, Token::LParen, Token::RParen)(rem)?;
        let (body, rem) = SubroutineBody::parse(rem)?;
        Ok((
            Box::new(Self {
                kind,
                type_,
                subroutine_name,
                params,
                body,
            }),
            rem,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassVarDec {
    pub kind: Box<ClassVarKind>,
    pub type_: Box<Type>,
    pub var_name: Box<Identifier>,
    pub extra_var_names: Box<Vec<Box<Identifier>>>,
}

impl Parsable for ClassVarDec {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (kind, rem) = ClassVarKind::parse(tokens)?;
        let (type_, rem) = Type::parse(rem)?;
        let (var_name, rem) = Identifier::parse(rem)?;
        let (extra_var_names, rem) =
            repeat_parser(drop1_parser(token_parser(Token::Comma), Identifier::parse))(rem)?;
        let (_, rem) = token_parser(Token::Semicolon)(rem)?;
        Ok((
            Box::new(Self {
                kind,
                type_,
                var_name,
                extra_var_names,
            }),
            rem,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub class_name: Box<Identifier>,
    pub var_decs: Box<Vec<Box<ClassVarDec>>>,
    pub subroutine_decs: Box<Vec<Box<SubroutineDec>>>,
}

impl Parsable for Class {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (_, rem) = token_parser(Token::Class)(tokens)?;
        let (class_name, rem) = Identifier::parse(rem)?;
        let (_, rem) = token_parser(Token::LBrace)(rem)?;
        let (var_decs, rem) = repeat_parser(ClassVarDec::parse)(rem)?;
        let (subroutine_decs, rem) = repeat_parser(SubroutineDec::parse)(rem)?;
        let (_, rem) = token_parser(Token::RBrace)(rem)?;
        Ok((
            Box::new(Self {
                class_name,
                var_decs,
                subroutine_decs,
            }),
            rem,
        ))
    }
}
