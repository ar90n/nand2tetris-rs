use anyhow::*;
use core::result::Result::Ok;

use super::token::Token;

fn skip_comments(s: &str) -> Result<&str> {
    let s = s.trim_start();
    let beg = if s.starts_with("//") {
        s.find('\n').unwrap_or(s.len())
    } else if s.starts_with("/*") {
        s.find("*/").with_context(|| "aaaf")? + 2
    } else {
        0
    };

    if 0 < beg {
        skip_comments(&s[beg..])
    } else {
        Ok(s)
    }
}

pub fn tokenize(s: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    let mut s = skip_comments(s)?;
    loop {
        let (token, rem) = Token::parse(s)?;
        if token == Token::EOF {
            break;
        }

        tokens.push(token);
        s = skip_comments(rem)?;
    }
    s = skip_comments(s)?;

    if s.is_empty() {
      Ok(tokens)
    } else {
        Err(anyhow::anyhow!("invalid token: {}", s))
    }
}

pub fn dump_as_xml(tokens: &[Token]) -> String {
    let mut tags = vec!["<tokens>".to_string()];
    for token in tokens {
        tags.push(token.dump_as_xml(0));
    }
    tags.push("</tokens>".to_string());
    tags.join("\n")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize(
                r#" (  )
{  }  -100  100  class  constructor  function  if  else  while
return  true  false  false_ _false // false
var  let  null  this true /* false  

aJ */
false
foo
/**/"#
            )
            .unwrap(),
            vec![
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::RBrace,
                Token::Minus,
                Token::IntegerConstant(100),
                Token::IntegerConstant(100),
                Token::Class,
                Token::Constructor,
                Token::Function,
                Token::If,
                Token::Else,
                Token::While,
                Token::Return,
                Token::True,
                Token::False,
                Token::Identifier("false_".to_string()),
                Token::Identifier("_false".to_string()),
                Token::Var,
                Token::Let,
                Token::Null,
                Token::This,
                Token::True,
                Token::False,
                Token::Identifier("foo".to_string()),
            ]
        );
    }
}
