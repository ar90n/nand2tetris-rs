use anyhow::*;

use crate::{
    parsable::repeat_parser,
    program::{Class, ParameterList},
};

use super::token::Token;

pub trait Parsable {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])>;
}

pub trait DumpXml {
    fn dump_as_xml(&self, level: usize) -> String;
    fn indent(&self, level: usize) -> String {
        "  ".repeat(level)
    }
    fn tag(&self, name: &str, level: usize) -> (String, String) {
        let indent = self.indent(level);
        let open_tag = format!("{}<{}>", indent, name);
        let close_tag = format!("{}</{}>", indent, name);
        (open_tag, close_tag)
    }
}

pub struct PlaceHolder {
    pub token: Token,
}

impl Parsable for PlaceHolder {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = tokens.split_first().context("tokens are empty")?;
        Ok((Box::new(Self { token: t.clone() }), rem))
    }
}

impl DumpXml for PlaceHolder {
    fn dump_as_xml(&self, level: usize) -> String {
        self.token.dump_as_xml(level)
    }
}

pub struct Empty {}

impl Parsable for Empty {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        Ok((Box::new(Self {}), tokens))
    }
}

impl DumpXml for Empty {
    fn dump_as_xml(&self, level: usize) -> String {
        String::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    True,
    False,
    Null,
    This,
    Integer(i16),
    String(String),
}

impl TryFrom<Token> for Constant {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::True => Ok(Self::True),
            Token::False => Ok(Self::False),
            Token::Null => Ok(Self::Null),
            Token::This => Ok(Self::This),
            Token::IntegerConstant(v) => Ok(Self::Integer(v)),
            Token::StringConstant(s) => Ok(Self::String(s)),
            _ => Err(anyhow!("invalid keyword constant: {:?}", token)),
        }
    }
}

impl Into<Token> for Constant {
    fn into(self) -> Token {
        match self {
            Self::True => Token::True,
            Self::False => Token::False,
            Self::Null => Token::Null,
            Self::This => Token::This,
            Self::Integer(v) => Token::IntegerConstant(v),
            Self::String(s) => Token::StringConstant(s),
        }
    }
}

impl Parsable for Constant {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = tokens.split_first().context("tokens are empty")?;
        let k = Self::try_from(t.clone())?;
        Ok((Box::new(k), rem))
    }
}

impl DumpXml for Constant {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Minus,
    Tilde,
}

impl TryFrom<Token> for UnaryOp {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Minus => Ok(Self::Minus),
            Token::Tilde => Ok(Self::Tilde),
            _ => Err(anyhow!("invalid unary op: {:?}", token)),
        }
    }
}

impl Into<Token> for UnaryOp {
    fn into(self) -> Token {
        match self {
            Self::Minus => Token::Minus,
            Self::Tilde => Token::Tilde,
        }
    }
}

impl Parsable for UnaryOp {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = tokens.split_first().context("tokens are empty")?;
        let k = Self::try_from(t.clone())?;
        Ok((Box::new(k), rem))
    }
}

impl DumpXml for UnaryOp {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Plus,
    Minus,
    Asterisk,
    Slash,
    And,
    Or,
    Lt,
    Gt,
    Eq,
}

impl TryFrom<Token> for Op {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Plus => Ok(Self::Plus),
            Token::Minus => Ok(Self::Minus),
            Token::Asterisk => Ok(Self::Asterisk),
            Token::Slash => Ok(Self::Slash),
            Token::And => Ok(Self::And),
            Token::Or => Ok(Self::Or),
            Token::Lt => Ok(Self::Lt),
            Token::Gt => Ok(Self::Gt),
            Token::Eq => Ok(Self::Eq),
            _ => Err(anyhow!("invalid op: {:?}", token)),
        }
    }
}

impl Into<Token> for Op {
    fn into(self) -> Token {
        match self {
            Self::Plus => Token::Plus,
            Self::Minus => Token::Minus,
            Self::Asterisk => Token::Asterisk,
            Self::Slash => Token::Slash,
            Self::And => Token::And,
            Self::Or => Token::Or,
            Self::Lt => Token::Lt,
            Self::Gt => Token::Gt,
            Self::Eq => Token::Eq,
        }
    }
}

impl Parsable for Op {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = tokens.split_first().context("tokens are empty")?;
        let k = Self::try_from(t.clone())?;
        Ok((Box::new(k), rem))
    }
}

impl DumpXml for Op {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(String);

impl TryFrom<Token> for Identifier {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Identifier(s) => Ok(Self(s)),
            _ => Err(anyhow!("invalid identifier: {:?}", token)),
        }
    }
}

impl Into<Token> for Identifier {
    fn into(self) -> Token {
        Token::Identifier(self.0)
    }
}

impl Parsable for Identifier {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = tokens.split_first().context("tokens are empty")?;
        let k = Self::try_from(t.clone())?;
        Ok((Box::new(k), rem))
    }
}

impl DumpXml for Identifier {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Char,
    Boolean,
    Class(String),
}

impl TryFrom<Token> for Type {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Int => Ok(Self::Int),
            Token::Char => Ok(Self::Char),
            Token::Boolean => Ok(Self::Boolean),
            Token::Identifier(s) => Ok(Self::Class(s)),
            _ => Err(anyhow!("invalid type: {:?}", token)),
        }
    }
}

impl Into<Token> for Type {
    fn into(self) -> Token {
        match self {
            Self::Int => Token::Int,
            Self::Char => Token::Char,
            Self::Boolean => Token::Boolean,
            Self::Class(s) => Token::Identifier(s),
        }
    }
}

impl Parsable for Type {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = tokens.split_first().context("tokens are empty")?;
        let k = Self::try_from(t.clone())?;
        Ok((Box::new(k), rem))
    }
}

impl DumpXml for Type {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassVarKind {
    Field,
    Static,
}

impl TryFrom<Token> for ClassVarKind {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Field => Ok(Self::Field),
            Token::Static => Ok(Self::Static),
            _ => Err(anyhow!("invalid class var kind: {:?}", token)),
        }
    }
}

impl Into<Token> for ClassVarKind {
    fn into(self) -> Token {
        match self {
            Self::Field => Token::Field,
            Self::Static => Token::Static,
        }
    }
}

impl Parsable for ClassVarKind {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = tokens.split_first().context("tokens are empty")?;
        let k = Self::try_from(t.clone())?;
        Ok((Box::new(k), rem))
    }
}

impl DumpXml for ClassVarKind {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

impl TryFrom<Token> for SubroutineKind {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Constructor => Ok(Self::Constructor),
            Token::Function => Ok(Self::Function),
            Token::Method => Ok(Self::Method),
            _ => Err(anyhow!("invalid subroutine kind: {:?}", token)),
        }
    }
}

impl Into<Token> for SubroutineKind {
    fn into(self) -> Token {
        match self {
            Self::Constructor => Token::Constructor,
            Self::Function => Token::Function,
            Self::Method => Token::Method,
        }
    }
}

impl Parsable for SubroutineKind {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = tokens.split_first().context("tokens are empty")?;
        let k = Self::try_from(t.clone())?;
        Ok((Box::new(k), rem))
    }
}

impl DumpXml for SubroutineKind {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubroutineType {
    Int,
    Char,
    Boolean,
    Void,
    Class(String),
}

impl TryFrom<Token> for SubroutineType {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self> {
        match token {
            Token::Int => Ok(Self::Int),
            Token::Char => Ok(Self::Char),
            Token::Boolean => Ok(Self::Boolean),
            Token::Void => Ok(Self::Void),
            Token::Identifier(s) => Ok(Self::Class(s)),
            _ => Err(anyhow!("invalid subroutine type: {:?}", token)),
        }
    }
}

impl Into<Token> for SubroutineType {
    fn into(self) -> Token {
        match self {
            Self::Int => Token::Int,
            Self::Char => Token::Char,
            Self::Boolean => Token::Boolean,
            Self::Void => Token::Void,
            Self::Class(s) => Token::Identifier(s),
        }
    }
}

impl Parsable for SubroutineType {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = tokens.split_first().context("tokens are empty")?;
        let k = Self::try_from(t.clone())?;
        Ok((Box::new(k), rem))
    }
}

impl DumpXml for SubroutineType {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection<T: Parsable> {
    pub items: Vec<Box<T>>,
}

impl<T: Parsable> Collection<T> {
    pub fn new(items: Vec<Box<T>>) -> Self {
        Self { items }
    }
}

impl<T: Parsable> Default for Collection<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T: Parsable> Parsable for Collection<T> {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (items, rem) = repeat_parser(T::parse)(tokens)?;
        Ok((Box::new(Self { items: *items }), rem))
    }
}

impl<T: Parsable + DumpXml> DumpXml for Collection<T> {
    fn dump_as_xml(&self, level: usize) -> String {
        self.items
            .iter()
            .map(|item| item.dump_as_xml(level))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optional<T: Parsable> {
    pub item: Option<Box<T>>,
}

impl<T: Parsable> Parsable for Optional<T> {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        if let anyhow::Result::Ok((item, rem_)) = T::parse(tokens) {
            Ok((Box::new(Self { item: Some(item) }), rem_))
        } else {
            Ok((Box::new(Self { item: None }), tokens))
        }
    }
}

impl<T: Parsable + DumpXml> DumpXml for Optional<T> {
    fn dump_as_xml(&self, level: usize) -> String {
        match &self.item {
            Some(item) => item.dump_as_xml(level),
            None => String::new(),
        }
    }
}

impl<T: Parsable, U: Parsable> Parsable for (Box<T>, Box<U>) {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = T::parse(tokens)?;
        let (u, rem_) = U::parse(rem)?;
        Ok((Box::new((t, u)), rem_))
    }
}

impl<T: Parsable + DumpXml, U: Parsable + DumpXml> DumpXml for (Box<T>, Box<U>) {
    fn dump_as_xml(&self, level: usize) -> String {
        let (t, u) = self;
        format!("{}\n{}", t.dump_as_xml(level), u.dump_as_xml(level))
    }
}

pub type Seq2<T: Parsable, U: Parsable> = (Box<T>, Box<U>);
pub type Seq3<T: Parsable, U: Parsable, V: Parsable> = (Box<T>, Box<U>, Box<V>);
pub type Seq4<T: Parsable, U: Parsable, V: Parsable, W: Parsable> =
    (Box<T>, Box<U>, Box<V>, Box<W>);

#[test]
fn test_constant() {
    assert_eq!(
        Constant::parse(&[Token::True]).unwrap(),
        (Box::new(Constant::True), [].as_slice())
    );

    assert_eq!(
        Constant::parse(&[Token::False, Token::True]).unwrap(),
        (Box::new(Constant::False), [Token::True].as_slice())
    );

    assert_eq!(
        Constant::parse(&[Token::Null]).unwrap(),
        (Box::new(Constant::Null), [].as_slice())
    );

    assert_eq!(
        Constant::parse(&[Token::This]).unwrap(),
        (Box::new(Constant::This), [].as_slice())
    );

    assert!(Constant::parse(&[Token::And]).is_err());
    assert!(Constant::parse(&[]).is_err());
}

#[test]
fn test_unary_op() {
    assert_eq!(
        UnaryOp::parse(&[Token::Minus]).unwrap(),
        (Box::new(UnaryOp::Minus), [].as_slice())
    );
    assert_eq!(
        UnaryOp::parse(&[Token::Tilde, Token::Minus]).unwrap(),
        (Box::new(UnaryOp::Tilde), [Token::Minus].as_slice())
    );
    assert!(UnaryOp::parse(&[Token::Plus]).is_err());
    assert!(UnaryOp::parse(&[]).is_err());
}

#[test]
fn test_op() {
    assert_eq!(
        Op::parse(&[Token::Plus]).unwrap(),
        (Box::new(Op::Plus), [].as_slice())
    );
    assert_eq!(
        Op::parse(&[Token::Minus]).unwrap(),
        (Box::new(Op::Minus), [].as_slice())
    );
    assert_eq!(
        Op::parse(&[Token::Asterisk]).unwrap(),
        (Box::new(Op::Asterisk), [].as_slice())
    );
    assert_eq!(
        Op::parse(&[Token::Slash]).unwrap(),
        (Box::new(Op::Slash), [].as_slice())
    );
    assert_eq!(
        Op::parse(&[Token::And]).unwrap(),
        (Box::new(Op::And), [].as_slice())
    );
    assert_eq!(
        Op::parse(&[Token::Or]).unwrap(),
        (Box::new(Op::Or), [].as_slice())
    );
    assert_eq!(
        Op::parse(&[Token::Lt]).unwrap(),
        (Box::new(Op::Lt), [].as_slice())
    );
    assert_eq!(
        Op::parse(&[Token::Gt]).unwrap(),
        (Box::new(Op::Gt), [].as_slice())
    );
    assert_eq!(
        Op::parse(&[Token::Eq, Token::Gt]).unwrap(),
        (Box::new(Op::Eq), [Token::Gt].as_slice())
    );
    assert!(Op::parse(&[Token::False]).is_err());
    assert!(Op::parse(&[]).is_err());
}
