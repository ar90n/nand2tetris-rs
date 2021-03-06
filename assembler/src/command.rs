use std::any;

use anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dest {
    None = 0,
    M = 1,
    D = 2,
    MD = 3,
    A = 4,
    AM = 5,
    AD = 6,
    AMD = 7,
}

impl Dest {
    pub fn from(bits: u8) -> anyhow::Result<Self> {
        match bits {
            0 => Ok(Dest::None),
            1 => Ok(Dest::M),
            2 => Ok(Dest::D),
            3 => Ok(Dest::MD),
            4 => Ok(Dest::A),
            5 => Ok(Dest::AM),
            6 => Ok(Dest::AD),
            7 => Ok(Dest::AMD),
            _ => Err(anyhow::anyhow!("Invalid dest bits: {}", bits)),
        }
    }

    pub fn parse(s: &str) -> anyhow::Result<(Self, &str)> {
        let s = s.trim();
        if let Some(end) = s.find('=') {
            let (dest, s) = s.split_at(end);
            match dest {
                "M" => Ok((Dest::M, s)),
                "D" => Ok((Dest::D, s)),
                "MD" => Ok((Dest::MD, s)),
                "A" => Ok((Dest::A, s)),
                "AM" => Ok((Dest::AM, s)),
                "AD" => Ok((Dest::AD, s)),
                "AMD" => Ok((Dest::AMD, s)),
                _ => Err(anyhow::anyhow!("invalid dest: {}", s)),
            }
        } else {
            Ok((Dest::None, s))
        }
    }

    pub fn assemble(self) -> u16 {
        self as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comp {
    ZERO = 0,
    ONE = 1,
    MINUS_ONE = 2,
    D = 3,
    A = 4,
    INV_D = 5,
    INV_A = 6,
    MINUS_D = 7,
    MINUS_A = 8,
    D_PLUS_ONE = 9,
    A_PLUS_ONE = 10,
    D_MINUS_ONE = 11,
    A_MINUS_ONE = 12,
    D_PLUS_A = 13,
    D_MINUS_A = 14,
    A_MINUS_D = 15,
    D_AND_A = 16,
    D_OR_A = 17,
    M = 18,
    INV_M = 19,
    MINUS_M = 20,
    M_PLUS_ONE = 21,
    M_MINUS_ONE = 22,
    D_PLUS_M = 23,
    D_MINUS_M = 24,
    M_MINUS_D = 25,
    D_AND_M = 26,
    D_OR_M = 27,
}

impl Comp {
    pub fn parse(s: &str) -> anyhow::Result<(Self, &str)> {
        let s = if s.starts_with('=') { &s[1..] } else { s };

        let terminals = vec![';', ' ', '\t', '/'];
        let end = s.find(|c| terminals.contains(&c)).unwrap_or(s.len());
        let (comp, s) = s.split_at(end);
        match comp {
            "0" => Ok((Comp::ZERO, s)),
            "1" => Ok((Comp::ONE, s)),
            "-1" => Ok((Comp::MINUS_ONE, s)),
            "D" => Ok((Comp::D, s)),
            "A" => Ok((Comp::A, s)),
            "!D" => Ok((Comp::INV_D, s)),
            "!A" => Ok((Comp::INV_A, s)),
            "-D" => Ok((Comp::MINUS_D, s)),
            "-A" => Ok((Comp::MINUS_A, s)),
            "D+1" => Ok((Comp::D_PLUS_ONE, s)),
            "A+1" => Ok((Comp::A_PLUS_ONE, s)),
            "D-1" => Ok((Comp::D_MINUS_ONE, s)),
            "A-1" => Ok((Comp::A_MINUS_ONE, s)),
            "D+A" => Ok((Comp::D_PLUS_A, s)),
            "D-A" => Ok((Comp::D_MINUS_A, s)),
            "A-D" => Ok((Comp::A_MINUS_D, s)),
            "D&A" => Ok((Comp::D_AND_A, s)),
            "D|A" => Ok((Comp::D_OR_A, s)),
            "M" => Ok((Comp::M, s)),
            "!M" => Ok((Comp::INV_M, s)),
            "-M" => Ok((Comp::MINUS_M, s)),
            "M+1" => Ok((Comp::M_PLUS_ONE, s)),
            "M-1" => Ok((Comp::M_MINUS_ONE, s)),
            "D+M" => Ok((Comp::D_PLUS_M, s)),
            "D-M" => Ok((Comp::D_MINUS_M, s)),
            "M-D" => Ok((Comp::M_MINUS_D, s)),
            "D&M" => Ok((Comp::D_AND_M, s)),
            "D|M" => Ok((Comp::D_OR_M, s)),
            _ => Err(anyhow::anyhow!("invalid comp: {}", comp)),
        }
    }

    pub fn assemble(self) -> u16 {
        match self {
            Comp::ZERO => 0x002a,
            Comp::ONE => 0x003f,
            Comp::MINUS_ONE => 0x0003a,
            Comp::D => 0x000c,
            Comp::A => 0x0030,
            Comp::INV_D => 0x000d,
            Comp::INV_A => 0x0031,
            Comp::MINUS_D => 0x000f,
            Comp::MINUS_A => 0x0033,
            Comp::D_PLUS_ONE => 0x001f,
            Comp::A_PLUS_ONE => 0x0037,
            Comp::D_MINUS_ONE => 0x000e,
            Comp::A_MINUS_ONE => 0x0032,
            Comp::D_PLUS_A => 0x0002,
            Comp::D_MINUS_A => 0x0013,
            Comp::A_MINUS_D => 0x0007,
            Comp::D_AND_A => 0x0000,
            Comp::D_OR_A => 0x0015,
            Comp::M => 0x0070,
            Comp::INV_M => 0x0071,
            Comp::MINUS_M => 0x0073,
            Comp::M_PLUS_ONE => 0x0077,
            Comp::M_MINUS_ONE => 0x0072,
            Comp::D_PLUS_M => 0x0042,
            Comp::D_MINUS_M => 0x0053,
            Comp::M_MINUS_D => 0x0047,
            Comp::D_AND_M => 0x0040,
            Comp::D_OR_M => 0x0055,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jump {
    None = 0,
    JGT = 1,
    JEQ = 2,
    JGE = 3,
    JLT = 4,
    JNE = 5,
    JLE = 6,
    JMP = 7,
}

impl Jump {
    pub fn from(bits: u16) -> anyhow::Result<Self> {
        match bits {
            0x0 => Ok(Jump::None),
            0x1 => Ok(Jump::JGT),
            0x2 => Ok(Jump::JEQ),
            0x3 => Ok(Jump::JGE),
            0x4 => Ok(Jump::JLT),
            0x5 => Ok(Jump::JNE),
            0x6 => Ok(Jump::JLE),
            0x7 => Ok(Jump::JMP),
            _ => Err(anyhow::anyhow!("Invalid jump instruction: {}", bits)),
        }
    }

    pub fn parse(s: &str) -> anyhow::Result<(Self, &str)> {
        let s = s.trim();
        let s = if s.starts_with(';') { &s[1..] } else { s };
        let terminals = vec![' ', '\t', '/'];
        let end = s.find(|c| terminals.contains(&c)).unwrap_or(s.len());

        let (jump, s) = s.split_at(end);
        match jump {
            "" => Ok((Jump::None, s)),
            "JGT" => Ok((Jump::JGT, s)),
            "JEQ" => Ok((Jump::JEQ, s)),
            "JGE" => Ok((Jump::JGE, s)),
            "JLT" => Ok((Jump::JLT, s)),
            "JNE" => Ok((Jump::JNE, s)),
            "JLE" => Ok((Jump::JLE, s)),
            "JMP" => Ok((Jump::JMP, s)),
            _ => anyhow::bail!("Invalid jump instruction: {}", jump),
        }
    }

    pub fn assemble(self) -> u16 {
        self as u16
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    AImm(u16),
    ASymbol(String),
    C(Dest, Comp, Jump),
    L(String),
}

impl Command {
    pub fn parse_a_command(s: &str) -> anyhow::Result<Self> {
        let s = s.trim();
        if s.is_empty() || !s.starts_with("@") {
            anyhow::bail!("Invalid A command: {}", s);
        }

        if let Ok(imm) = s[1..].parse::<u16>() {
            Ok(Command::AImm(imm))
        } else {
            Ok(Command::ASymbol(s[1..].to_string()))
        }
    }

    pub fn parse_c_command(s: &str) -> anyhow::Result<Self> {
        let s = s.trim();
        if s.is_empty() {
            anyhow::bail!("Invalid C command: {}", s);
        }

        Dest::parse(s).and_then(|(dest, s)| {
            Comp::parse(s)
                .and_then(|(comp, s)| Jump::parse(s).map(|(jump, _)| Command::C(dest, comp, jump)))
        })
    }

    pub fn parse_l_command(s: &str) -> anyhow::Result<Self> {
        let s = s.trim();
        if s.is_empty() || !s.starts_with("(") || !s.ends_with(")") {
            anyhow::bail!("Invalid L command: {}", s);
        }

        let label = &s[1..s.len() - 1];
        Ok(Command::L(label.to_string()))
    }

    pub fn parse(s: &str) -> anyhow::Result<Self> {
        Command::parse_a_command(s)
            .or_else(|_| Command::parse_c_command(s))
            .or_else(|_| Command::parse_l_command(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_jump() {
        let (jump, count) = Jump::parse("JGT").unwrap();
        assert_eq!(jump, Jump::JGT);
        assert_eq!(count, "");

        let (jump, count) = Jump::parse("JMP  ").unwrap();
        assert_eq!(jump, Jump::JMP);
        assert_eq!(count, "");

        let (jump, count) = Jump::parse(";JMP").unwrap();
        assert_eq!(jump, Jump::JMP);
        assert_eq!(count, "");

        let (jump, count) = Jump::parse("JMP //foo").unwrap();
        assert_eq!(jump, Jump::JMP);
        assert_eq!(count, " //foo");

        let (jump, count) = Jump::parse("").unwrap();
        assert_eq!(jump, Jump::None);
        assert_eq!(count, "");

        let err = Jump::parse("JM");
        assert!(err.is_err());
    }

    #[test]
    fn test_parse_dest() {
        let (dest, count) = Dest::parse("M=1").unwrap();
        assert_eq!(dest, Dest::M);
        assert_eq!(count, "=1");

        let (dest, count) = Dest::parse("1").unwrap();
        assert_eq!(dest, Dest::None);
        assert_eq!(count, "1");

        let (dest, count) = Dest::parse("AMD=2").unwrap();
        assert_eq!(dest, Dest::AMD);
        assert_eq!(count, "=2");
    }

    #[test]
    fn test_parse_comp() {
        let (comp, count) = Comp::parse("0").unwrap();
        assert_eq!(comp, Comp::ZERO);
        assert_eq!(count, "");

        let (comp, count) = Comp::parse("M+1").unwrap();
        assert_eq!(comp, Comp::M_PLUS_ONE);
        assert_eq!(count, "");

        let (comp, count) = Comp::parse("D|M ").unwrap();
        assert_eq!(comp, Comp::D_OR_M);
        assert_eq!(count, " ");

        let (comp, count) = Comp::parse("D&M//foo").unwrap();
        assert_eq!(comp, Comp::D_AND_M);
        assert_eq!(count, "//foo");

        let (comp, count) = Comp::parse("-1;").unwrap();
        assert_eq!(comp, Comp::MINUS_ONE);
        assert_eq!(count, ";");
    }

    #[test]
    fn test_parse() {
        let command = Command::parse("@1").unwrap();
        assert_eq!(command, Command::AImm(1));

        let command = Command::parse("@Loop").unwrap();
        assert_eq!(command, Command::ASymbol("Loop".to_string()));

        let command = Command::parse("D=0").unwrap();
        assert_eq!(command, Command::C(Dest::D, Comp::ZERO, Jump::None));

        let command = Command::parse("D=M  /").unwrap();
        assert_eq!(command, Command::C(Dest::D, Comp::M, Jump::None));

        let command = Command::parse("0;JMP").unwrap();
        assert_eq!(command, Command::C(Dest::None, Comp::ZERO, Jump::JMP));

        let command = Command::parse("M=D+A;JGT").unwrap();
        assert_eq!(command, Command::C(Dest::M, Comp::D_PLUS_A, Jump::JGT));

        let command = Command::parse("(LOOP)").unwrap();
        assert_eq!(command, Command::L("LOOP".to_string()));
    }
}
