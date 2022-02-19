use anyhow::*;

fn trim_start(s: &str) -> &str {
    let beg = s
        .find(|c: char| !c.is_whitespace() && c != '\n')
        .unwrap_or(s.len());
    &s[beg..]
}

fn is_delimiter(c: char) -> bool {
    c.is_whitespace() || c.is_control() || c == ';'
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Period,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    And,
    Or,
    Lt,
    Gt,
    Eq,
    Tilde,
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    IntegerConstant(i16),
    StringConstant(String),
    Identifier(String),
    EOF,
}

impl Token {
    fn parse_symbol(s: &str) -> Result<(Self, &str)> {
        let (s, rem) = s.split_at(1);
        match s {
            "(" => Ok((Self::LParen, rem)),
            ")" => Ok((Self::RParen, rem)),
            "{" => Ok((Self::LBrace, rem)),
            "}" => Ok((Self::RBrace, rem)),
            "[" => Ok((Self::LBracket, rem)),
            "]" => Ok((Self::RBracket, rem)),
            "." => Ok((Self::Period, rem)),
            "," => Ok((Self::Comma, rem)),
            ";" => Ok((Self::Semicolon, rem)),
            "+" => Ok((Self::Plus, rem)),
            "-" => Ok((Self::Minus, rem)),
            "*" => Ok((Self::Asterisk, rem)),
            "/" => Ok((Self::Slash, rem)),
            "&" => Ok((Self::And, rem)),
            "|" => Ok((Self::Or, rem)),
            "<" => Ok((Self::Lt, rem)),
            ">" => Ok((Self::Gt, rem)),
            "=" => Ok((Self::Eq, rem)),
            "~" => Ok((Self::Tilde, rem)),
            _ => Err(anyhow!("invalid symbol: {}", s)),
        }
    }

    fn parse_keyword(s: &str) -> Result<(Self, &str)> {
        let end = s
            .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .unwrap_or(s.len());
        let (s, rem) = s.split_at(end);
        match s {
            "class" => Ok((Self::Class, rem)),
            "constructor" => Ok((Self::Constructor, rem)),
            "function" => Ok((Self::Function, rem)),
            "method" => Ok((Self::Method, rem)),
            "field" => Ok((Self::Field, rem)),
            "static" => Ok((Self::Static, rem)),
            "var" => Ok((Self::Var, rem)),
            "int" => Ok((Self::Int, rem)),
            "char" => Ok((Self::Char, rem)),
            "boolean" => Ok((Self::Boolean, rem)),
            "void" => Ok((Self::Void, rem)),
            "true" => Ok((Self::True, rem)),
            "false" => Ok((Self::False, rem)),
            "null" => Ok((Self::Null, rem)),
            "this" => Ok((Self::This, rem)),
            "let" => Ok((Self::Let, rem)),
            "do" => Ok((Self::Do, rem)),
            "if" => Ok((Self::If, rem)),
            "else" => Ok((Self::Else, rem)),
            "while" => Ok((Self::While, rem)),
            "return" => Ok((Self::Return, rem)),
            _ => Err(anyhow!("invalid keyword: {}", s)),
        }
    }
    fn parse_integer_constant(s: &str) -> Result<(Self, &str)> {
        let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
        let (s, rem) = s.split_at(end);
        let n = s
            .parse::<i16>()
            .map_err(|_| anyhow!("invalid integer constant: {}", s))?;

        Ok((Self::IntegerConstant(n), rem))
    }

    fn parse_string_constant(s: &str) -> Result<(Self, &str)> {
        if !s.starts_with("\"") {
            return Err(anyhow!("invalid string constant: {}", s));
        }

        match s[1..].find('"') {
            Some(end) => {
                let s = &s[1..];
                let (s, rem) = s.split_at(end);
                let rem = &rem[1..];
                Ok((Self::StringConstant(s.to_string()), rem))
            }
            None => Err(anyhow!("invalid string constant: {}", s)),
        }
    }
    fn parse_identifier(s: &str) -> Result<(Self, &str)> {
        if s.starts_with(|c: char| c.is_ascii_digit()) {
            return Err(anyhow!("invalid identifier: {}", s));
        }

        let end = s
            .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .unwrap_or(s.len());
        let (s, rem) = s.split_at(end);
        Ok((Self::Identifier(s.to_string()), rem))
    }

    pub fn parse(s: &str) -> Result<(Self, &str)> {
        if s.is_empty() {
            return Ok((Self::EOF, s));
        }

        Self::parse_keyword(s)
            .or_else(|_| Self::parse_symbol(s))
            .or_else(|_| Self::parse_integer_constant(s))
            .or_else(|_| Self::parse_string_constant(s))
            .or_else(|_| Self::parse_identifier(s))
    }

    pub fn dump_as_xml(&self, level: usize) -> String {
        let mut ret = "  ".repeat(level);
        ret += &match self {
            Self::LParen => "<symbol> ( </symbol>".to_string(),
            Self::RParen => "<symbol> ) </symbol>".to_string(),
            Self::LBrace => "<symbol> { </symbol>".to_string(),
            Self::RBrace => "<symbol> } </symbol>".to_string(),
            Self::LBracket => "<symbol> [ </symbol>".to_string(),
            Self::RBracket => "<symbol> ] </symbol>".to_string(),
            Self::Period => "<symbol> . </symbol>".to_string(),
            Self::Comma => "<symbol> , </symbol>".to_string(),
            Self::Semicolon => "<symbol> ; </symbol>".to_string(),
            Self::Plus => "<symbol> + </symbol>".to_string(),
            Self::Minus => "<symbol> - </symbol>".to_string(),
            Self::Asterisk => "<symbol> * </symbol>".to_string(),
            Self::Slash => "<symbol> / </symbol>".to_string(),
            Self::And => "<symbol> &amp; </symbol>".to_string(),
            Self::Or => "<symbol> | </symbol>".to_string(),
            Self::Lt => "<symbol> &lt; </symbol>".to_string(),
            Self::Gt => "<symbol> &gt; </symbol>".to_string(),
            Self::Eq => "<symbol> = </symbol>".to_string(),
            Self::Tilde => "<symbol> ~ </symbol>".to_string(),
            Self::Class => "<keyword> class </keyword>".to_string(),
            Self::Constructor => "<keyword> constructor </keyword>".to_string(),
            Self::Function => "<keyword> function </keyword>".to_string(),
            Self::Method => "<keyword> method </keyword>".to_string(),
            Self::Field => "<keyword> field </keyword>".to_string(),
            Self::Static => "<keyword> static </keyword>".to_string(),
            Self::Var => "<keyword> var </keyword>".to_string(),
            Self::Int => "<keyword> int </keyword>".to_string(),
            Self::Char => "<keyword> char </keyword>".to_string(),
            Self::Boolean => "<keyword> boolean </keyword>".to_string(),
            Self::Void => "<keyword> void </keyword>".to_string(),
            Self::True => "<keyword> true </keyword>".to_string(),
            Self::False => "<keyword> false </keyword>".to_string(),
            Self::Null => "<keyword> null </keyword>".to_string(),
            Self::This => "<keyword> this </keyword>".to_string(),
            Self::Let => "<keyword> let </keyword>".to_string(),
            Self::Do => "<keyword> do </keyword>".to_string(),
            Self::If => "<keyword> if </keyword>".to_string(),
            Self::Else => "<keyword> else </keyword>".to_string(),
            Self::While => "<keyword> while </keyword>".to_string(),
            Self::Return => "<keyword> return </keyword>".to_string(),
            Self::Identifier(s) => format!("<identifier> {} </identifier>", s),
            Self::StringConstant(s) => format!("<stringConstant> {} </stringConstant>", s),
            Self::IntegerConstant(n) => format!("<integerConstant> {} </integerConstant>", n),
            Self::EOF => String::default(),
        };
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol() {
        assert_eq!(Token::parse(")  ").unwrap(), (Token::RParen, "  "));
        assert_eq!(Token::parse("{100}").unwrap(), (Token::LBrace, "100}"));
        assert_eq!(Token::parse("-100").unwrap(), (Token::Minus, "100"));
        assert_eq!(Token::parse("}{").unwrap(), (Token::RBrace, "{"));
    }

    #[test]
    fn test_keyword() {
        assert_eq!(Token::parse("class").unwrap(), (Token::Class, ""));
        assert_eq!(
            Token::parse("constructor  ").unwrap(),
            (Token::Constructor, "  ")
        );
        assert_eq!(
            Token::parse("function abc").unwrap(),
            (Token::Function, " abc")
        );
        assert_eq!(Token::parse("return;").unwrap(), (Token::Return, ";"));
        assert_eq!(Token::parse("this)").unwrap(), (Token::This, ")"));
    }

    #[test]
    fn test_integer_constant() {
        assert_eq!(
            Token::parse("100").unwrap(),
            (Token::IntegerConstant(100), "")
        );
        assert_eq!(
            Token::parse("100 aa").unwrap(),
            (Token::IntegerConstant(100), " aa")
        );
        assert_eq!(
            Token::parse("100a").unwrap(),
            (Token::IntegerConstant(100), "a")
        );
        assert_eq!(
            Token::parse("100.0").unwrap(),
            (Token::IntegerConstant(100), ".0")
        );
    }

    #[test]
    fn test_string_constant() {
        assert_eq!(
            Token::parse("\"abc\"").unwrap(),
            (Token::StringConstant("abc".to_string()), "")
        );
        assert_eq!(
            Token::parse("\"abc\"  ").unwrap(),
            (Token::StringConstant("abc".to_string()), "  ")
        );
        assert_eq!(
            Token::parse("\"abc\\\"\"  ").unwrap(),
            (Token::StringConstant("abc\\".to_string()), "\"  ")
        );
        assert_eq!(
            Token::parse("\"abc\"\"def\"").unwrap(),
            (Token::StringConstant("abc".to_string()), "\"def\"")
        );
    }

    #[test]
    fn test_identifier() {
        assert_eq!(
            Token::parse("abc def").unwrap(),
            (Token::Identifier("abc".to_string()), " def")
        );
        assert_eq!(
            Token::parse("abc_def").unwrap(),
            (Token::Identifier("abc_def".to_string()), "")
        );
        assert_eq!(
            Token::parse("abc-def").unwrap(),
            (Token::Identifier("abc".to_string()), "-def")
        );
        assert_eq!(
            Token::parse("if2").unwrap(),
            (Token::Identifier("if2".to_string()), "")
        );
        assert_eq!(
            Token::parse("if_").unwrap(),
            (Token::Identifier("if_".to_string()), "")
        );
        assert_eq!(
            Token::parse("_if").unwrap(),
            (Token::Identifier("_if".to_string()), "")
        );
    }

    #[test]
    fn test_eof() {
        assert_eq!(Token::parse("").unwrap(), (Token::EOF, ""));
    }
}
