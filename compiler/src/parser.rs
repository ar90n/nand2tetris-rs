use super::program::Class;
use super::foundation::Parsable;
use super::token::Token;

use anyhow::*;

pub fn parse(tokens: &[Token]) -> Result<Box<Class>> {
    let (c, _) = Class::parse(tokens)?;
    Ok(c)
}
