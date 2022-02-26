use super::expression::*;
use super::foundation::*;
use super::program::*;
use super::statement::*;
use super::token::Token;

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

impl DumpXml for Token {
    fn dump_as_xml(&self, level: usize) -> String {
        let mut ret = self.indent(level);
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

impl DumpXml for PlaceHolder {
    fn dump_as_xml(&self, level: usize) -> String {
        self.token.dump_as_xml(level)
    }
}

impl DumpXml for Empty {
    fn dump_as_xml(&self, _: usize) -> String {
        String::default()
    }
}

impl DumpXml for UnaryOp {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

impl DumpXml for Op {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}
impl DumpXml for Identifier {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

impl DumpXml for Constant {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

impl DumpXml for Type {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}
impl DumpXml for ClassVarKind {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}

impl DumpXml for SubroutineKind {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
    }
}
impl DumpXml for SubroutineType {
    fn dump_as_xml(&self, level: usize) -> String {
        let t: Token = self.clone().into();
        t.dump_as_xml(level)
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
impl<T: Parsable + DumpXml> DumpXml for Optional<T> {
    fn dump_as_xml(&self, level: usize) -> String {
        match &self.item {
            Some(item) => item.dump_as_xml(level),
            None => String::new(),
        }
    }
}

impl<T: Parsable + DumpXml, U: Parsable + DumpXml> DumpXml for (Box<T>, Box<U>) {
    fn dump_as_xml(&self, level: usize) -> String {
        let (t, u) = self;
        format!("{}\n{}", t.dump_as_xml(level), u.dump_as_xml(level))
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
impl DumpXml for SubroutineCall {
    fn dump_as_xml(&self, level: usize) -> String {
        match self {
            Self::ExternalCall(var_name, method_name, args) => {
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
            Self::InternalCall(function_name, args) => {
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

impl DumpXml for VarDec {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("varDec", level);
        let mut tags = vec![
            open_tag,
            Token::Var.dump_as_xml(level + 1),
            self.type_.dump_as_xml(level + 1),
        ];

        self.var_names.iter().enumerate().for_each(|(i, x)| {
            if 0 < i {
                tags.push(Token::Comma.dump_as_xml(level + 1));
            }
            tags.push(x.dump_as_xml(level + 1));
        });

        tags.extend([Token::Semicolon.dump_as_xml(level + 1), close_tag]);
        tags.join("\n")
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

impl DumpXml for ClassVarDec {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("classVarDec", level);
        let mut tags = vec![
            open_tag,
            self.kind.dump_as_xml(level + 1),
            self.type_.dump_as_xml(level + 1),
        ];
        self.var_names.iter().enumerate().for_each(|(i, x)| {
            if 0 < i {
                tags.push(Token::Comma.dump_as_xml(level + 1));
            }
            tags.push(x.dump_as_xml(level + 1));
        });
        tags.push(Token::Semicolon.dump_as_xml(level + 1));
        tags.push(close_tag);
        tags.join("\n")
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
impl DumpXml for ReturnStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("returnStatement", level);
        [
            open_tag,
            Token::Return.dump_as_xml(level + 1),
            self.0.dump_as_xml(level + 1),
            Token::Semicolon.dump_as_xml(level + 1),
            close_tag,
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
    }
}
impl DumpXml for DoStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("doStatement", level);
        vec![
            open_tag,
            Token::Do.dump_as_xml(level + 1),
            self.0.dump_as_xml(level + 1),
            Token::Semicolon.dump_as_xml(level + 1),
            close_tag,
        ]
        .join("\n")
    }
}
impl DumpXml for WhileStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("whileStatement", level);
        vec![
            open_tag,
            Token::While.dump_as_xml(level + 1),
            Token::LParen.dump_as_xml(level + 1),
            self.condition.dump_as_xml(level + 1),
            Token::RParen.dump_as_xml(level + 1),
            Token::LBrace.dump_as_xml(level + 1),
            self.body.dump_as_xml(level + 1),
            Token::RBrace.dump_as_xml(level + 1),
            close_tag,
        ]
        .join("\n")
    }
}
impl DumpXml for IfStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("ifStatement", level);
        let mut tags = vec![
            open_tag,
            Token::If.dump_as_xml(level + 1),
            Token::LParen.dump_as_xml(level + 1),
            self.condition.dump_as_xml(level + 1),
            Token::RParen.dump_as_xml(level + 1),
            Token::LBrace.dump_as_xml(level + 1),
            self.then_body.dump_as_xml(level + 1),
            Token::RBrace.dump_as_xml(level + 1),
        ];
        if let Some(ref v) = *self.else_body {
            tags.push(Token::Else.dump_as_xml(level + 1));
            tags.push(Token::LBrace.dump_as_xml(level + 1));
            tags.push(v.dump_as_xml(level + 1));
            tags.push(Token::RBrace.dump_as_xml(level + 1));
        }
        tags.push(close_tag);
        tags.join("\n")
    }
}
impl DumpXml for LetStatement {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("letStatement", level);
        let mut tags = vec![
            open_tag,
            Token::Let.dump_as_xml(level + 1),
            self.var_name.dump_as_xml(level + 1),
        ];

        if let Some(ref v) = *self.array_size {
            tags.extend([
                Token::LBracket.dump_as_xml(level + 1),
                v.dump_as_xml(level + 1),
                Token::RBracket.dump_as_xml(level + 1),
            ]);
        }

        tags.extend([
            Token::Eq.dump_as_xml(level + 1),
            self.expression.dump_as_xml(level + 1),
            Token::Semicolon.dump_as_xml(level + 1),
            close_tag,
        ]);
        tags.join("\n")
    }
}
impl DumpXml for Statement {
    fn dump_as_xml(&self, level: usize) -> String {
        match self {
            Self::Let(v) => v.dump_as_xml(level),
            Self::If(v) => v.dump_as_xml(level),
            Self::While(v) => v.dump_as_xml(level),
            Self::Do(v) => v.dump_as_xml(level),
            Self::Return(v) => v.dump_as_xml(level),
        }
    }
}
impl DumpXml for Statements {
    fn dump_as_xml(&self, level: usize) -> String {
        let (open_tag, close_tag) = self.tag("statements", level);
        [open_tag, self.statements.dump_as_xml(level + 1), close_tag]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
