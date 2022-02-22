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

impl DumpXml for VarDec {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("varDec", level);
        let mut tags = vec![
            open_tag,
            Token::Var.dump_as_xml(level + 1),
            self.type_.dump_as_xml(level + 1),
            self.var_name.dump_as_xml(level + 1),
        ];

        self.extra_var_names.iter().for_each(|x| {
            tags.push(Token::Comma.dump_as_xml(level + 1));
            tags.push(x.dump_as_xml(level + 1));
        });

        tags.extend([Token::Semicolon.dump_as_xml(level + 1), close_tag]);
        tags.join("\n")
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

impl DumpXml for SubroutineBody {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("subroutineBody", level);
        let mut tags = vec![open_tag, Token::LBrace.dump_as_xml(level + 1)];

        self.var_decs.iter().for_each(|x| {
            tags.push(x.dump_as_xml(level + 1));
        });

        tags.extend([
            self.statements.dump_as_xml(level + 1),
            Token::RBrace.dump_as_xml(level + 1),
            close_tag,
        ]);

        tags.join("\n")
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

impl DumpXml for ParameterList {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("parameterList", level);
        let mut tags = vec![open_tag];

        self.params.iter().enumerate().for_each(|(i, pair)| {
            if 0 < i {
                tags.push(Token::Comma.dump_as_xml(level + 1));
            }
            tags.push(pair.0.dump_as_xml(level + 1));
            tags.push(pair.1.dump_as_xml(level + 1));
        });

        tags.push(close_tag);
        tags.join("\n")
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

impl DumpXml for SubroutineDec {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("subroutineDec", level);
        vec![
            open_tag,
            self.kind.dump_as_xml(level + 1),
            self.type_.dump_as_xml(level + 1),
            self.subroutine_name.dump_as_xml(level + 1),
            Token::LParen.dump_as_xml(level + 1),
            self.params.dump_as_xml(level + 1),
            Token::RParen.dump_as_xml(level + 1),
            self.body.dump_as_xml(level + 1),
            close_tag,
        ]
        .join("\n")
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

impl DumpXml for ClassVarDec {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("classVarDec", level);
        let mut tags = vec![
            open_tag,
            self.kind.dump_as_xml(level + 1),
            self.type_.dump_as_xml(level + 1),
            self.var_name.dump_as_xml(level + 1),
        ];
        self.extra_var_names.iter().for_each(|x| {
            tags.push(Token::Comma.dump_as_xml(level + 1));
            tags.push(x.dump_as_xml(level + 1));
        });
        tags.push(Token::Semicolon.dump_as_xml(level + 1));
        tags.push(close_tag);
        tags.join("\n")
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

impl DumpXml for Class {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("class", level);
        let mut tags = vec![
            open_tag,
            Token::Class.dump_as_xml(level + 1),
            self.class_name.dump_as_xml(level + 1),
            Token::LBrace.dump_as_xml(level + 1),
        ];
        for v in self.var_decs.iter() {
            tags.push(v.dump_as_xml(level + 1));
        }
        for s in self.subroutine_decs.iter() {
            tags.push(s.dump_as_xml(level + 1));
        }
        tags.push(Token::RBrace.dump_as_xml(level + 1));
        tags.push(close_tag);
        tags.join("\n")
    }
}
