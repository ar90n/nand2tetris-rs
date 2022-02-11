use rand::{distributions::Alphanumeric, Rng};

use super::command::{Command as VmCommand, Segment};
use assembler::code::assemble;
use assembler::command::{Command, Comp, Dest, Jump};

pub fn parse(lines: &[String]) -> anyhow::Result<Vec<VmCommand>> {
    lines
        .iter()
        .filter(|line| {
            let line = line.trim();
            !line.is_empty() && !line.starts_with("//")
        })
        .map(|s| VmCommand::parse(&s))
        .collect::<anyhow::Result<Vec<_>>>()
}

fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn translate_vmcommand(vm_command: &VmCommand) -> Vec<Command> {
    fn translate_unary_command(f: impl Fn() -> Vec<Command>) -> Vec<Command> {
        let mut result = Vec::new();
        result.extend(vec![
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::A, Comp::M, Jump::None),
            Command::C(Dest::A, Comp::A_MINUS_ONE, Jump::None),
            Command::C(Dest::D, Comp::M, Jump::None),
        ]);
        result.extend(f());
        result.extend(vec![
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::A, Comp::M, Jump::None),
            Command::C(Dest::A, Comp::A_MINUS_ONE, Jump::None),
            Command::C(Dest::M, Comp::D, Jump::None),
        ]);
        result
    }

    fn translate_binary_command(f: impl Fn() -> Vec<Command>) -> Vec<Command> {
        let mut result = Vec::new();
        result.extend(vec![
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::A, Comp::M, Jump::None),
            Command::C(Dest::A, Comp::A_MINUS_ONE, Jump::None),
            Command::C(Dest::D, Comp::M, Jump::None),
            Command::C(Dest::A, Comp::A_MINUS_ONE, Jump::None),
            Command::C(Dest::A, Comp::M, Jump::None),
        ]);
        result.extend(f());
        result.extend(vec![
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::A, Comp::M, Jump::None),
            Command::C(Dest::A, Comp::A_MINUS_ONE, Jump::None),
            Command::C(Dest::A, Comp::A_MINUS_ONE, Jump::None),
            Command::C(Dest::M, Comp::D, Jump::None),
            Command::C(Dest::D, Comp::A, Jump::None),
            Command::C(Dest::D, Comp::D_PLUS_ONE, Jump::None),
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::M, Comp::D, Jump::None),
        ]);
        result
    }

    fn translate_condition_command(jump: Jump) -> Vec<Command> {
        let l0 = random_string(12);
        let l1 = random_string(12);
        vec![
            Command::C(Dest::D, Comp::A_MINUS_D, Jump::None),
            Command::ASymbol(l0.clone()),
            Command::C(Dest::None, Comp::D, jump),
            Command::ASymbol(l1.clone()),
            Command::C(Dest::D, Comp::ZERO, Jump::JMP),
            Command::L(l0.clone()),
            Command::C(Dest::D, Comp::MINUS_ONE, Jump::None),
            Command::L(l1.clone()),
        ]
    }

    fn push_d_reg() -> Vec<Command> {
        vec![
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::A, Comp::M, Jump::None),
            Command::C(Dest::M, Comp::D, Jump::None),
            Command::C(Dest::D, Comp::A_PLUS_ONE, Jump::None),
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::M, Comp::D, Jump::None),
        ]
    }

    fn pop_d_reg() -> Vec<Command> {
        vec![
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::A, Comp::M, Jump::None),
            Command::C(Dest::D, Comp::A_MINUS_ONE, Jump::None),
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::M, Comp::D, Jump::None),
            Command::C(Dest::A, Comp::D, Jump::None),
            Command::C(Dest::D, Comp::M, Jump::None),
        ]
    }

    fn load_segment(symbol_name: String, index: u16, offset: u16) -> Vec<Command> {
        let mut commands = vec![
            Command::AImm(index),
            Command::C(Dest::D, Comp::A, Jump::None),
            //Command::ASymbol(symbol_name),
            (if offset == 0 || offset == 0xffff {
                Command::ASymbol(symbol_name.clone())
            } else {
                Command::AImm(offset)
            }),
            (if offset == 0 {
                Command::C(Dest::A, Comp::M, Jump::None)
            } else if offset == 0xffff {
                Command::ASymbol(symbol_name.clone())
            } else {
                Command::AImm(offset)
            }),
            Command::C(Dest::A, Comp::D_PLUS_A, Jump::None),
            Command::C(Dest::D, Comp::M, Jump::None),
        ];
        commands.extend(push_d_reg());
        commands
    }
    fn store_segment(symbol_name: String, index: u16, offset: u16) -> Vec<Command> {
        let mut commands = vec![
            // D <- symbol + index
            Command::AImm(index),
            Command::C(Dest::D, Comp::A, Jump::None),
            (if offset == 0 || offset == 0xffff {
                Command::ASymbol(symbol_name.clone())
            } else {
                Command::AImm(offset)
            }),
            (if offset == 0 {
                Command::C(Dest::A, Comp::M, Jump::None)
            } else if offset == 0xffff {
                Command::ASymbol(symbol_name.clone())
            } else {
                Command::AImm(offset)
            }),
            Command::C(Dest::D, Comp::D_PLUS_A, Jump::None),
            // next <- symbol + index
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::A, Comp::M, Jump::None),
            Command::C(Dest::M, Comp::D, Jump::None),
            // D <- top
            Command::C(Dest::A, Comp::A_MINUS_ONE, Jump::None),
            Command::C(Dest::D, Comp::M, Jump::None),
            // A <- symbol + index
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::A, Comp::M, Jump::None),
            Command::C(Dest::A, Comp::M, Jump::None),
            // symbol + index <- top
            Command::C(Dest::M, Comp::D, Jump::None),
            // A <- SP
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::A, Comp::M, Jump::None),
            // D <- SP - 1
            Command::C(Dest::D, Comp::A_MINUS_ONE, Jump::None),
            // A <- SP
            Command::ASymbol("SP".to_string()),
            Command::C(Dest::M, Comp::D, Jump::None),
        ];
        commands
    }

    match vm_command {
        VmCommand::Push(segment, index) => match segment {
            Segment::Argument => load_segment("ARG".to_string(), *index, 0),
            Segment::Local => load_segment("LCL".to_string(), *index, 0),
            Segment::This => load_segment("THIS".to_string(), *index, 0),
            Segment::That => load_segment("THAT".to_string(), *index, 0),
            Segment::Pointer => load_segment("SP".to_string(), *index, 3),
            Segment::Temp => load_segment("SP".to_string(), *index, 5),
            Segment::Static => {
                let symbol_name = format!("__STATIC.{}", index);
                let mut commands = load_segment(symbol_name, 0, 0xffff);
                commands
            }
            Segment::Constant => {
                let mut commands = vec![
                    Command::AImm(*index),
                    Command::C(Dest::D, Comp::A, Jump::None),
                ];
                commands.extend(push_d_reg());
                commands
            }
            _ => vec![],
        },
        VmCommand::Pop(segment, index) => match segment {
            Segment::Argument => store_segment("ARG".to_string(), *index, 0),
            Segment::Local => store_segment("LCL".to_string(), *index, 0),
            Segment::This => store_segment("THIS".to_string(), *index, 0),
            Segment::That => store_segment("THAT".to_string(), *index, 0),
            Segment::Pointer => store_segment("SP".to_string(), *index, 3),
            Segment::Temp => store_segment("SP".to_string(), *index, 5),
            Segment::Static => {
                let symbol_name = format!("__STATIC.{}", index);
                let mut commands = store_segment(symbol_name, 0, 0xffff);
                commands
            }
            Segment::Constant => {
                vec![
                    Command::ASymbol("SP".to_string()),
                    Command::C(Dest::A, Comp::M, Jump::None),
                    Command::C(Dest::D, Comp::A_MINUS_ONE, Jump::None),
                    Command::ASymbol("SP".to_string()),
                    Command::C(Dest::M, Comp::D, Jump::None),
                ]
            }
            _ => vec![],
        },

        VmCommand::Add => {
            translate_binary_command(|| vec![Command::C(Dest::D, Comp::D_PLUS_A, Jump::None)])
        }
        VmCommand::Sub => {
            translate_binary_command(|| vec![Command::C(Dest::D, Comp::A_MINUS_D, Jump::None)])
        }
        VmCommand::Neg => {
            translate_unary_command(|| vec![Command::C(Dest::D, Comp::MINUS_D, Jump::None)])
        }
        VmCommand::Eq => translate_binary_command(|| translate_condition_command(Jump::JEQ)),
        VmCommand::Gt => translate_binary_command(|| translate_condition_command(Jump::JGT)),
        VmCommand::Lt => translate_binary_command(|| translate_condition_command(Jump::JLT)),
        VmCommand::And => {
            translate_binary_command(|| vec![Command::C(Dest::D, Comp::D_AND_A, Jump::None)])
        }
        VmCommand::Or => {
            translate_binary_command(|| vec![Command::C(Dest::D, Comp::D_OR_A, Jump::None)])
        }
        VmCommand::Not => {
            translate_unary_command(|| vec![Command::C(Dest::D, Comp::INV_D, Jump::None)])
        }
        _ => panic!("aaaa"),
    }
}

pub fn translate(vm_commands: &[VmCommand]) -> anyhow::Result<Vec<Command>> {
    let mut result = Vec::new();
    for vm_command in vm_commands {
        result.extend(translate_vmcommand(vm_command));
    }
    Ok(result)
}
