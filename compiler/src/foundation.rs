use anyhow::*;

use super::token::Token;

pub trait Parsable {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])>;
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

pub struct Empty {}

impl Parsable for Empty {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        Ok((Box::new(Self {}), tokens))
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


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection<T: Parsable> {
    pub items: Vec<Box<T>>,
}

impl<T: Parsable> Collection<T> {
    pub fn new(items: Vec<Box<T>>) -> Self {
        Self { items }
    }

    pub fn len(&self) -> usize {
        self.items.len()
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


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optional<T: Parsable> {
    pub item: Option<Box<T>>,
}

impl<T: Parsable> Optional<T> {
    pub fn new(item: Option<Box<T>>) -> Self {
        Self { item }
    }
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

impl<T: Parsable, U: Parsable> Parsable for (Box<T>, Box<U>) {
    fn parse(tokens: &[Token]) -> Result<(Box<Self>, &[Token])> {
        let (t, rem) = T::parse(tokens)?;
        let (u, rem_) = U::parse(rem)?;
        Ok((Box::new((t, u)), rem_))
    }
}


pub type Seq2<T, U> = (Box<T>, Box<U>);
pub type Seq3<T, U, V> = (Box<T>, Box<U>, Box<V>);
pub type Seq4<T, U, V, W> = (Box<T>, Box<U>, Box<V>, Box<W>);

pub(crate) fn token_parser(
    target: Token,
) -> impl Fn(&[Token]) -> Result<(Box<PlaceHolder>, &[Token])> {
    move |tokens: &[Token]| {
        if tokens.is_empty() {
            return Err(anyhow!("tokens are empty"));
        }

        let (token, rem) = tokens.split_at(1);
        if token[0] == target {
            Ok((
                Box::new(PlaceHolder {
                    token: target.clone(),
                }),
                rem,
            ))
        } else {
            Err(anyhow!("invalid token: {:?}", token))
        }
    }
}

pub(crate) fn option_parser<T>(
    parser: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<Option<Box<T>>>, &[Token])> {
    move |tokens: &[Token]| {
        if let anyhow::Result::Ok((t, tokens)) = parser(tokens) {
            Ok((Box::new(Some(t)), tokens))
        } else {
            Ok((Box::new(None), tokens))
        }
    }
}

pub(crate) fn repeat_parser<T>(
    parser: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<Vec<Box<T>>>, &[Token])> {
    move |tokens: &[Token]| {
        let mut result = vec![];
        let mut tokens = tokens;
        while let anyhow::Result::Ok((item, rem)) = parser(tokens) {
            result.push(item);
            tokens = rem;
        }
        Ok((Box::new(result), tokens))
    }
}

pub(crate) fn seq2_parser<T, U>(
    parser_t: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
    parser_u: impl Fn(&[Token]) -> Result<(Box<U>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<(Box<T>, Box<U>)>, &[Token])> {
    move |tokens: &[Token]| {
        let (t, tokens) = parser_t(tokens)?;
        let (u, tokens) = parser_u(tokens)?;
        Ok((Box::new((t, u)), tokens))
    }
}

pub(crate) fn take1_parser<T, U>(
    parser_t: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
    parser_u: impl Fn(&[Token]) -> Result<(Box<U>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<T>, &[Token])> {
    move |tokens: &[Token]| {
        let (t, tokens) = parser_t(tokens)?;
        let (_, tokens) = parser_u(tokens)?;
        Ok((t, tokens))
    }
}


pub(crate) fn drop1_parser<T, U>(
    parser_t: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
    parser_u: impl Fn(&[Token]) -> Result<(Box<U>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<U>, &[Token])> {
    move |tokens: &[Token]| {
        let (_, tokens) = parser_t(tokens)?;
        let (u, tokens) = parser_u(tokens)?;
        Ok((u, tokens))
    }
}

pub(crate) fn surround_parser<T>(
    parser: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
    surround_left: Token,
    surround_right: Token,
) -> impl Fn(&[Token]) -> Result<(Box<T>, &[Token])> {
    let parser_left = token_parser(surround_left);
    let parser_right = token_parser(surround_right);
    move |tokens: &[Token]| {
        let (_, tokens) = parser_left(tokens)?;
        let (t, tokens) = parser(tokens)?;
        let (_, tokens) = parser_right(tokens)?;
        Ok((t, tokens))
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
