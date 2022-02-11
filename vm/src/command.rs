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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl Command {
    fn parse_segment_and_index(s: &str) -> anyhow::Result<(Segment, u16)> {
        let (segment, s) = Segment::parse(s)?;

        let s = s.trim();
        let end = s.find(' ').unwrap_or(s.len());
        let (index, _) = s.split_at(end);
        let index = index.parse::<u16>()?;
        Ok((segment, index))
    }

    pub fn parse(s: &str) -> anyhow::Result<Self> {
        let s = s.trim();
        let end = s.find(' ').unwrap_or(s.len());
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
            _ => Err(anyhow::anyhow!("Unknown command: {}", command)),
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
