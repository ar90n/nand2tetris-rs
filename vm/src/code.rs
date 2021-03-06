use std::io::BufRead;

use rand::{distributions::Alphanumeric, Rng};

use super::command::{Command as VmCommand, Segment};
use assembler::command::{Command, Comp, Dest, Jump};

pub fn parse(lines: &[&str]) -> anyhow::Result<Vec<VmCommand>> {
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

fn translate_vmcommand(
    vm_command: &VmCommand,
    filename: &str,
    funcname: String,
) -> (Vec<Command>, String) {
    let push = [
        Command::ASymbol("SP".to_string()),
        Command::C(Dest::A, Comp::M, Jump::None),
        Command::C(Dest::M, Comp::D, Jump::None),
        Command::ASymbol("SP".to_string()),
        Command::C(Dest::M, Comp::M_PLUS_ONE, Jump::None),
    ];

    let pop = [
        Command::ASymbol("SP".to_string()),
        Command::C(Dest::M, Comp::M_MINUS_ONE, Jump::None),
        Command::ASymbol("SP".to_string()),
        Command::C(Dest::A, Comp::M, Jump::None),
        Command::C(Dest::A, Comp::M, Jump::None),
    ];

    let swap = [
        Command::C(Dest::D, Comp::D_PLUS_A, Jump::None),
        Command::C(Dest::A, Comp::D_MINUS_A, Jump::None),
        Command::C(Dest::D, Comp::D_MINUS_A, Jump::None),
    ];

    let translate_unary_command = |c: Command| -> Vec<Command> {
        let mut result = Vec::new();
        result.extend(pop.clone());
        result.push(Command::C(Dest::D, Comp::A, Jump::None));
        result.extend([c]);
        result.extend(push.clone());
        result
    };

    let translate_binary_command = |cs: Vec<Command>| -> Vec<Command> {
        let mut result = Vec::new();
        result.extend(pop.clone());
        result.push(Command::C(Dest::D, Comp::A, Jump::None));
        result.extend(pop.clone());
        result.extend(cs);
        result.extend(push.clone());
        result
    };

    fn assign_imm_if(jump: Jump, true_value: Comp, false_value: Comp) -> Vec<Command> {
        let l0 = random_string(12);
        let l1 = random_string(12);
        vec![
            Command::C(Dest::D, Comp::A_MINUS_D, Jump::None),
            Command::ASymbol(l0.clone()),
            Command::C(Dest::None, Comp::D, jump),
            Command::ASymbol(l1.clone()),
            Command::C(Dest::D, false_value, Jump::JMP),
            Command::L(l0.clone()),
            Command::C(Dest::D, true_value, Jump::None),
            Command::L(l1.clone()),
        ]
    }

    let calc_address = |symbol_name: String, index: u16| -> Vec<Command> {
        vec![
            Command::AImm(index),
            Command::C(Dest::D, Comp::A, Jump::None),
            Command::ASymbol(symbol_name.clone()),
            Command::C(Dest::A, Comp::D_PLUS_M, Jump::None),
        ]
    };

    let load_segment = |calc_address_commands: Vec<Command>| -> Vec<Command> {
        let mut commands = calc_address_commands;
        commands.push(Command::C(Dest::D, Comp::M, Jump::None));
        commands.extend(push.clone());
        commands
    };

    let store_segment = |calc_address_commands: Vec<Command>| -> Vec<Command> {
        let mut commands = calc_address_commands;
        commands.push(Command::C(Dest::D, Comp::A, Jump::None));
        commands.extend(pop.clone());
        commands.extend(swap.clone());
        commands.push(Command::C(Dest::M, Comp::D, Jump::None));
        commands
    };

    let mut funcname = funcname;
    let commands = match vm_command {
        VmCommand::Push(segment, index) => match segment {
            Segment::Argument => load_segment(calc_address("ARG".to_string(), *index)),
            Segment::Local => load_segment(calc_address("LCL".to_string(), *index)),
            Segment::This => load_segment(calc_address("THIS".to_string(), *index)),
            Segment::That => load_segment(calc_address("THAT".to_string(), *index)),
            Segment::Pointer => load_segment(vec![Command::AImm(*index + 3)]),
            Segment::Temp => load_segment(vec![Command::AImm(*index + 5)]),
            Segment::Static => {
                load_segment(vec![Command::ASymbol(format!("{}.{}", filename, index))])
            }
            Segment::Constant => {
                let mut commands = vec![
                    Command::AImm(*index),
                    Command::C(Dest::D, Comp::A, Jump::None),
                ];
                commands.extend(push.clone());
                commands
            }
            _ => panic!("unsupported segment for push: {:?}", segment),
        },
        VmCommand::Pop(segment, index) => match segment {
            Segment::Argument => store_segment(calc_address("ARG".to_string(), *index)),
            Segment::Local => store_segment(calc_address("LCL".to_string(), *index)),
            Segment::This => store_segment(calc_address("THIS".to_string(), *index)),
            Segment::That => store_segment(calc_address("THAT".to_string(), *index)),
            Segment::Pointer => store_segment(vec![Command::AImm(*index + 3)]),
            Segment::Temp => store_segment(vec![Command::AImm(*index + 5)]),
            Segment::Static => {
                store_segment(vec![Command::ASymbol(format!("{}.{}", filename, index))])
            }
            Segment::Constant => {
                vec![
                    Command::ASymbol("SP".to_string()),
                    Command::C(Dest::M, Comp::M_MINUS_ONE, Jump::None),
                ]
            }
            _ => panic!("unsupported segment for pop: {:?}", segment),
        },

        VmCommand::Add => {
            translate_binary_command(vec![Command::C(Dest::D, Comp::D_PLUS_A, Jump::None)])
        }
        VmCommand::Sub => {
            translate_binary_command(vec![Command::C(Dest::D, Comp::A_MINUS_D, Jump::None)])
        }
        VmCommand::Neg => translate_unary_command(Command::C(Dest::D, Comp::MINUS_D, Jump::None)),
        VmCommand::Eq => {
            translate_binary_command(assign_imm_if(Jump::JEQ, Comp::MINUS_ONE, Comp::ZERO))
        }
        VmCommand::Gt => {
            translate_binary_command(assign_imm_if(Jump::JGT, Comp::MINUS_ONE, Comp::ZERO))
        }
        VmCommand::Lt => {
            translate_binary_command(assign_imm_if(Jump::JLT, Comp::MINUS_ONE, Comp::ZERO))
        }
        VmCommand::And => {
            translate_binary_command(vec![Command::C(Dest::D, Comp::D_AND_A, Jump::None)])
        }
        VmCommand::Or => {
            translate_binary_command(vec![Command::C(Dest::D, Comp::D_OR_A, Jump::None)])
        }
        VmCommand::Not => translate_unary_command(Command::C(Dest::D, Comp::INV_D, Jump::None)),
        VmCommand::Label(ref label) => {
            vec![Command::L(format!("{}.{}", funcname, label))]
        }
        VmCommand::Function(ref label, k) => {
            funcname = label.clone();
            vec![
                Command::L(label.clone()),
                Command::ASymbol("SP".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
                Command::ASymbol("LCL".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
                Command::AImm(*k),
                Command::C(Dest::D, Comp::D_PLUS_A, Jump::None),
                Command::ASymbol("SP".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
            ]
        }
        VmCommand::Return => {
            let mut commands = pop.clone().to_vec();
            commands.extend([
                Command::C(Dest::D, Comp::A, Jump::None),
                Command::ASymbol("R13".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
                Command::ASymbol("ARG".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
                Command::ASymbol("R16".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
                Command::ASymbol("LCL".to_string()),
                Command::C(Dest::A, Comp::M, Jump::None),
                Command::C(Dest::D, Comp::A, Jump::None),
                Command::ASymbol("SP".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
            ]);
            commands.extend(pop.clone());
            commands.extend([
                Command::C(Dest::D, Comp::A, Jump::None),
                Command::ASymbol("THAT".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
            ]);
            commands.extend(pop.clone());
            commands.extend([
                Command::C(Dest::D, Comp::A, Jump::None),
                Command::ASymbol("THIS".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
            ]);
            commands.extend(pop.clone());
            commands.extend([
                Command::C(Dest::D, Comp::A, Jump::None),
                Command::ASymbol("ARG".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
            ]);
            commands.extend(pop.clone());
            commands.extend([
                Command::C(Dest::D, Comp::A, Jump::None),
                Command::ASymbol("LCL".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
            ]);
            commands.extend(pop.clone());
            commands.extend([
                Command::C(Dest::D, Comp::A, Jump::None),
                Command::ASymbol("R14".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
            ]);
            commands.extend([
                Command::ASymbol("R16".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
                Command::ASymbol("SP".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
                Command::ASymbol("R13".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
            ]);
            commands.extend(push.clone());
            commands.extend([
                Command::ASymbol("R14".to_string()),
                Command::C(Dest::A, Comp::M, Jump::None),
                Command::C(Dest::None, Comp::ZERO, Jump::JMP),
            ]);
            commands
        }

        VmCommand::Call(ref label, k) => {
            let return_address = random_string(12);
            let mut commands = vec![];
            commands.extend([
                Command::ASymbol(return_address.clone()),
                Command::C(Dest::D, Comp::A, Jump::None),
            ]);
            commands.extend(push.clone());
            commands.extend([
                Command::ASymbol("LCL".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
            ]);
            commands.extend(push.clone());
            commands.extend([
                Command::ASymbol("ARG".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
            ]);
            commands.extend(push.clone());
            commands.extend([
                Command::ASymbol("THIS".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
            ]);
            commands.extend(push.clone());
            commands.extend([
                Command::ASymbol("THAT".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
            ]);
            commands.extend(push.clone());
            commands.extend([
                Command::ASymbol("SP".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
                Command::AImm(k + 5),
                Command::C(Dest::D, Comp::D_MINUS_A, Jump::None),
                Command::ASymbol("ARG".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
            ]);
            commands.extend([
                Command::ASymbol("SP".to_string()),
                Command::C(Dest::D, Comp::M, Jump::None),
                Command::ASymbol("LCL".to_string()),
                Command::C(Dest::M, Comp::D, Jump::None),
            ]);
            commands.extend([
                Command::ASymbol(label.clone()),
                Command::C(Dest::None, Comp::ZERO, Jump::JMP),
                Command::L(return_address.clone())
            ]);
            commands
        }

        VmCommand::Goto(ref label) => {
            vec![
                Command::ASymbol(format!("{}.{}", funcname, label)),
                Command::C(Dest::None, Comp::ZERO, Jump::JMP),
            ]
        }
        VmCommand::IfGoto(ref label) => {
            let mut commands = vec![];
            commands.extend(pop.clone());
            commands.extend([
                Command::C(Dest::D, Comp::A, Jump::None),
                Command::ASymbol(format!("{}.{}", funcname, label)),
                Command::C(Dest::None, Comp::D, Jump::JNE),
            ]);
            commands
        }
        _ => panic!("unsupported command"),
    };
    (commands, funcname)
}

pub fn bootstrap(sp: u16) -> Vec<Command> {
    vec![
        Command::AImm(sp),
        Command::C(Dest::D, Comp::A, Jump::None),
        Command::ASymbol("SP".to_string()),
        Command::C(Dest::M, Comp::D, Jump::None),
        Command::ASymbol("Sys.init".to_string()),
        Command::C(Dest::None, Comp::ZERO, Jump::JMP),
    ]
}

pub fn translate(
    vm_commands: &[VmCommand],
    filename: Option<&str>,
) -> anyhow::Result<Vec<Command>> {
    let mut result = Vec::new();
    let filename = filename.unwrap_or("__STATIC");
    let mut funcname = String::default();
    for vm_command in vm_commands {
        let (c, f2) = translate_vmcommand(vm_command, filename, funcname);
        funcname = f2;
        result.extend(c);
    }
    Ok(result)
}
