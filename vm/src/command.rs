use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

impl Segment {
    pub fn parse(s: &str) -> anyhow::Result<(Self, &str)> {
        let s = s.trim();
        let end = s.find(' ').unwrap_or(s.len());
        let (segment, s) = s.split_at(end);
        match segment {
            "argument" => Ok((Segment::Argument, s)),
            "local" => Ok((Segment::Local, s)),
            "static" => Ok((Segment::Static, s)),
            "constant" => Ok((Segment::Constant, s)),
            "this" => Ok((Segment::This, s)),
            "that" => Ok((Segment::That, s)),
            "pointer" => Ok((Segment::Pointer, s)),
            "temp" => Ok((Segment::Temp, s)),
            _ => Err(anyhow::anyhow!("Unknown segment: {}", segment)),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = match self {
            Self::Argument => "argument",
            Self::Local => "local",
            Self::Static => "static",
            Self::Constant => "constant",
            Self::This => "this",
            Self::That => "that",
            Self::Pointer => "pointer",
            Self::Temp => "temp",
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Push(Segment, u16),
    Pop(Segment, u16),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Function(String, u16),
    Call(String, u16),
    Return,
    Label(String),
    Goto(String),
    IfGoto(String),
}

impl Command {
    fn parse_segment_and_index(s: &str) -> anyhow::Result<(Segment, u16)> {
        let (segment, s) = Segment::parse(s)?;

        let s = s.trim();
        let end = s.find(|c| c == ' ' || c == '\t').unwrap_or(s.len());
        let (index, _) = s.split_at(end);
        let index = index.parse::<u16>()?;
        Ok((segment, index))
    }

    fn parse_label(s: &str) -> anyhow::Result<(String, &str)> {
        let s = s.trim();
        let end = s.find(|c| c == ' ' || c == '\t').unwrap_or(s.len());
        let (label, s) = s.split_at(end);
        Ok((label.to_string(), s))
    }

    fn parse_function_args(s: &str) -> anyhow::Result<(String, u16)> {
        let s = s.trim();
        let end = s
            .find(|c| c == ' ' || c == '\t')
            .ok_or(anyhow::anyhow!("No space found in {} 1", s))?;
        let (name, s) = s.split_at(end);
        let s = s.trim();
        let end = s.find(|c| c == ' ' || c == '\t').unwrap_or(s.len());
        let (arg_count, _) = s.split_at(end);
        let arg_count = arg_count.parse::<u16>()?;
        Ok((name.to_string(), arg_count))
    }

    pub fn parse(s: &str) -> anyhow::Result<Self> {
        let s = s.trim();
        let end = s.find(|c| c == ' ' || c == '\t').unwrap_or(s.len());
        let (command, s) = s.split_at(end);
        match command {
            "push" => {
                let (segment, index) = Command::parse_segment_and_index(s)?;
                Ok(Command::Push(segment, index))
            }
            "pop" => {
                let (segment, index) = Command::parse_segment_and_index(s)?;
                Ok(Command::Pop(segment, index))
            }
            "add" => Ok(Command::Add),
            "sub" => Ok(Command::Sub),
            "neg" => Ok(Command::Neg),
            "eq" => Ok(Command::Eq),
            "gt" => Ok(Command::Gt),
            "lt" => Ok(Command::Lt),
            "and" => Ok(Command::And),
            "or" => Ok(Command::Or),
            "not" => Ok(Command::Not),
            "function" => {
                let (name, arg_count) = Command::parse_function_args(s)?;
                Ok(Command::Function(name, arg_count))
            }
            "call" => {
                let (name, arg_count) = Command::parse_function_args(s)?;
                Ok(Command::Call(name, arg_count))
            }
            "return" => Ok(Command::Return),
            "label" => {
                let (s, _) = Command::parse_label(s)?;
                Ok(Command::Label(s))
            }
            "goto" => {
                let (s, _) = Command::parse_label(s)?;
                Ok(Command::Goto(s))
            }
            "if-goto" => {
                let (s, _) = Command::parse_label(s)?;
                Ok(Command::IfGoto(s))
            }
            _ => Err(anyhow::anyhow!("Unknown command: {}", command)),
        }
    }

    pub fn dump(&self) -> String {
        match self {
            Command::Push(segment, index) => format!("push {} {}", segment, index),
            Command::Pop(segment, index) => format!("pop {} {}", segment, index),
            Command::Add => "add".to_string(),
            Command::Sub => "sub".to_string(),
            Command::Neg => "neg".to_string(),
            Command::Eq => "eq".to_string(),
            Command::Gt => "gt".to_string(),
            Command::Lt => "lt".to_string(),
            Command::And => "and".to_string(),
            Command::Or => "or".to_string(),
            Command::Not => "not".to_string(),
            Command::Function(name, arg_count) => {
                format!("function {} {}", name, arg_count)
            }
            Command::Call(name, arg_count) => format!("call {} {}", name, arg_count),
            Command::Return => "return".to_string(),
            Command::Label(label) => format!("label {}", label),
            Command::Goto(label) => format!("goto {}", label),
            Command::IfGoto(label) => format!("if-goto {}", label),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_segment() {
        let (segment, s) = Segment::parse("argument").unwrap();
        assert_eq!(segment, Segment::Argument);
        assert_eq!(s, "");

        let (segment, s) = Segment::parse("pointer 1 //foo").unwrap();
        assert_eq!(segment, Segment::Pointer);
        assert_eq!(s, " 1 //foo");
    }

    #[test]
    fn test_command() {
        let command = Command::parse("push argument 1 //foo").unwrap();
        assert_eq!(command, Command::Push(Segment::Argument, 1));

        let command = Command::parse("add //foo").unwrap();
        assert_eq!(command, Command::Add);
    }
}
