use anyhow::Context as _;
use anyhow::*;
use rand::{distributions::Alphanumeric, Rng};

use std::ops::Deref;

use super::foundation::*;
use super::program::*;
use crate::expression::*;
use crate::statement::*;
use vm::command::{Command, Segment};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType {
    Int,
    Char,
    Boolean,
    Class(String),
}

impl From<Type> for SymbolType {
    fn from(t: Type) -> Self {
        match t {
            Type::Int => SymbolType::Int,
            Type::Char => SymbolType::Char,
            Type::Boolean => SymbolType::Boolean,
            Type::Class(name) => SymbolType::Class(name),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Static,
    Field,
    Argument,
    Var,
}

impl From<ClassVarKind> for SymbolKind {
    fn from(kind: ClassVarKind) -> Self {
        match kind {
            ClassVarKind::Static => SymbolKind::Static,
            ClassVarKind::Field => SymbolKind::Field,
        }
    }
}

impl From<SymbolKind> for Segment {
    fn from(kind: SymbolKind) -> Self {
        match kind {
            SymbolKind::Static => Segment::Static,
            SymbolKind::Field => Segment::This,
            SymbolKind::Argument => Segment::Argument,
            SymbolKind::Var => Segment::Local,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolAttribute {
    pub name: String,
    pub type_: SymbolType,
    pub kind: SymbolKind,
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolTable {
    class_scope: Vec<SymbolAttribute>,
    method_scope: Vec<SymbolAttribute>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            class_scope: Vec::new(),
            method_scope: Vec::new(),
        }
    }

    pub fn start_subroutine(&mut self) {
        self.method_scope.clear();
    }

    pub fn define_class_variable(&mut self, name: String, type_: SymbolType, kind: SymbolKind) {
        let index = self.get_class_variable_count(kind);
        self.class_scope.push(SymbolAttribute {
            name,
            type_,
            kind,
            index,
        });
    }

    pub fn define_method_variable(&mut self, name: String, type_: SymbolType, kind: SymbolKind) {
        let index = self.get_method_variable_count(kind);
        self.method_scope.push(SymbolAttribute {
            name,
            type_,
            kind,
            index,
        });
    }

    pub fn get(&self, name: &str) -> Option<SymbolAttribute> {
        self.method_scope
            .iter()
            .find(|attr| attr.name == name)
            .or_else(|| self.class_scope.iter().find(|attr| attr.name == name))
            .cloned()
    }

    pub fn get_class_variable_count(&self, kind: SymbolKind) -> usize {
        self.class_scope
            .iter()
            .filter(|attr| attr.kind == kind)
            .count()
    }

    pub fn get_method_variable_count(&self, kind: SymbolKind) -> usize {
        self.method_scope
            .iter()
            .filter(|attr| attr.kind == kind)
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    pub class_name: String,
    symbol_table: SymbolTable,
    subroutine_name: Option<String>,
    subroutine_kind: Option<SubroutineKind>,
}

impl Context {
    pub fn new(class_name: String) -> Self {
        let symbol_table = SymbolTable::new();
        Context {
            class_name,
            symbol_table,
            subroutine_name: None,
            subroutine_kind: None,
        }
    }

    pub fn start_subroutine(&mut self, name: String, kind: SubroutineKind) {
        self.symbol_table.start_subroutine();
        self.subroutine_name = Some(name);
        self.subroutine_kind = Some(kind);
    }

    pub fn define_class_variable(&mut self, name: String, type_: SymbolType, kind: SymbolKind) {
        self.symbol_table.define_class_variable(name, type_, kind)
    }

    pub fn define_method_variable(&mut self, name: String, type_: SymbolType, kind: SymbolKind) {
        self.symbol_table.define_method_variable(name, type_, kind)
    }

    pub fn get(&self, name: &str) -> Option<SymbolAttribute> {
        self.symbol_table.get(name)
    }

    pub fn get_function_name(&self) -> Result<String> {
        let subroutine_name = self
            .subroutine_name
            .as_ref()
            .context("subroutine name is not defined")?;
        Ok(format!("{}.{}", self.class_name, subroutine_name))
    }

    pub fn get_function_kind(&self) -> Result<SubroutineKind> {
        self.subroutine_kind
            .context("subroutine kind is not defined")
    }

    pub fn get_local_variables(&self) -> Vec<SymbolAttribute> {
        self.symbol_table
            .method_scope
            .iter()
            .filter(|attr| attr.kind == SymbolKind::Var)
            .cloned()
            .collect()
    }

    pub fn get_local_variable_count(&self) -> usize {
        self.symbol_table.get_method_variable_count(SymbolKind::Var)
    }

    pub fn get_instance_variable_count(&self) -> usize {
        self.symbol_table
            .get_class_variable_count(SymbolKind::Field)
    }
}

pub trait DumpVm {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command>;
}

impl DumpVm for ParameterList {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        self.params.iter().cloned().for_each(|pair| {
            let (type_, identifier) = *pair;
            let type_ = (*type_).into();
            let Identifier(name) = *identifier;
            context.define_method_variable(name.clone(), type_, SymbolKind::Argument);
        });

        vec![]
    }
}

impl DumpVm for VarDec {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        self.var_names.iter().cloned().for_each(|identifier| {
            let type_ = self.type_.deref().clone().into();
            let Identifier(name) = *identifier;
            context.define_method_variable(name.clone(), type_, SymbolKind::Var);
        });

        vec![]
    }
}

impl DumpVm for Constant {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        match self {
            Constant::Null => {
                vec![Command::Push(Segment::Constant, 0)]
            }
            Constant::Integer(value) => {
                vec![Command::Push(Segment::Constant, *value as u16)]
            }
            Constant::True => {
                vec![Command::Push(Segment::Constant, 0), Command::Not]
            }
            Constant::False => {
                vec![Command::Push(Segment::Constant, 0)]
            }
            Constant::This => {
                vec![Command::Push(Segment::Pointer, 0)]
            }
            Constant::String(ref value) => {
                if context.get(value).is_none() {
                    context.define_method_variable(
                        value.clone(),
                        SymbolType::Class("String".to_string()),
                        SymbolKind::Var,
                    );
                }
                let attr = context.get(value).unwrap();
                vec![Command::Push(Segment::Local, attr.index as u16)]
            }
        }
    }
}

impl DumpVm for Term {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        match self {
            Term::Constant(value) => value.dump_as_vm(context),
            Term::Variable(identifier) => {
                let Identifier(ref name) = **identifier;
                let symbol = context.symbol_table.get(&name).unwrap();
                let segment = symbol.kind.into();
                vec![Command::Push(segment, symbol.index as u16)]
            }
            Term::SubroutineCall(subroutine_call) => subroutine_call.dump_as_vm(context),
            Term::Expression(expr) => expr.dump_as_vm(context),
            Term::UnaryOp(op, term) => {
                let mut commands = term.dump_as_vm(context);
                let op_command = match **op {
                    UnaryOp::Minus => Command::Neg,
                    UnaryOp::Tilde => Command::Not,
                };
                commands.push(op_command);
                commands
            }
            Term::ArrayAccess(identifier, index) => {
                let Identifier(ref name) = **identifier;
                let attr = context.get(name).unwrap();
                let segment = attr.kind.into();
                let mut commands = index.dump_as_vm(context);
                commands.extend([
                    Command::Push(segment, attr.index as u16),
                    Command::Add,
                    Command::Pop(Segment::Pointer, 1),
                    Command::Push(Segment::That, 0),
                ]);
                commands
            }
        }
    }
}

impl DumpVm for Op {
    fn dump_as_vm(&self, _context: &mut Context) -> Vec<Command> {
        match self {
            Op::Plus => vec![Command::Add],
            Op::Minus => vec![Command::Sub],
            Op::Asterisk => vec![Command::Call("Math.multiply".to_string(), 2)],
            Op::Slash => vec![Command::Call("Math.divide".to_string(), 2)],
            Op::Lt => vec![Command::Lt],
            Op::Gt => vec![Command::Gt],
            Op::Eq => vec![Command::Eq],
            Op::And => vec![Command::And],
            Op::Or => vec![Command::Or],
        }
    }
}

impl DumpVm for Expression {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let mut commands = self.term.dump_as_vm(context);
        for (op, term) in self.extras.items.iter().map(|t| t.deref()) {
            commands.extend(term.dump_as_vm(context));
            commands.extend(op.dump_as_vm(context));
        }
        commands
    }
}

impl DumpVm for LetStatement {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let Identifier(ref name) = self.var_name.deref();
        let symbol = context.get(name).unwrap();
        let segment = symbol.kind.into();

        let mut commands = vec![];

        // evaluate right hand
        let push_expression_result = self.expression.dump_as_vm(context);
        commands.extend(push_expression_result);

        if let Some(index_expression) = self.array_size.deref() {
            let push_array_size_result = index_expression.dump_as_vm(context);
            commands.extend(push_array_size_result);
            commands.extend(vec![
                Command::Push(segment, symbol.index as u16),
                Command::Add,
                Command::Pop(Segment::Pointer, 1),
                Command::Pop(Segment::That, 0),
            ])
        } else {
            commands.extend(vec![Command::Pop(segment, symbol.index as u16)])
        }

        commands
    }
}

impl DumpVm for IfStatement {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let then_label = random_string(16);
        let end_label = random_string(16);

        let mut commands = self.condition.dump_as_vm(context);
        commands.push(Command::IfGoto(then_label.clone()));

        if let Some(else_body) = self.else_body.deref() {
            commands.extend(else_body.dump_as_vm(context));
        }
        commands.push(Command::Goto(end_label.clone()));

        commands.push(Command::Label(then_label));
        commands.extend(self.then_body.dump_as_vm(context));

        commands.push(Command::Label(end_label));
        commands
    }
}

fn random_string(len: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    format!("_{}", s)
}

impl DumpVm for WhileStatement {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let loop_beg_label = random_string(8);
        let loop_end_label = random_string(8);

        let mut commands = vec![Command::Label(loop_beg_label.clone())];

        commands.extend(self.condition.dump_as_vm(context));
        commands.extend([Command::Not, Command::IfGoto(loop_end_label.clone())]);

        commands.extend(self.body.dump_as_vm(context));
        commands.extend([
            Command::Goto(loop_beg_label.clone()),
            Command::Label(loop_end_label.clone()),
        ]);

        commands
    }
}

impl DumpVm for ExpressionList {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        self.expressions.dump_as_vm(context)
    }
}

impl DumpVm for SubroutineCall {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let mut commands = vec![];
        match self {
            Self::ExternalCall(obj_name, func_name, args) => match context.get(&obj_name.0) {
                Some(symbol) => match symbol.type_ {
                    SymbolType::Class(ref class) => {
                        let segment = symbol.kind.into();
                        commands.push(Command::Push(segment, symbol.index as u16));
                        commands.extend(args.dump_as_vm(context));
                        let func_name = format!("{}.{}", class, func_name.0);
                        commands.push(Command::Call(func_name, args.len() as u16 + 1));
                    }
                    _ => panic!("unsupported symbol type {:?}", symbol.name),
                },
                None => {
                    commands.extend(args.dump_as_vm(context));
                    let func_name = format!("{}.{}", obj_name.0, func_name.0);
                    commands.push(Command::Call(func_name, args.len() as u16));
                }
            },
            Self::InternalCall(func_name, args) => {
                commands.push(Command::Push(Segment::Pointer, 0));
                commands.extend(args.dump_as_vm(context));
                let func_name = format!("{}.{}", context.class_name, func_name.0);
                commands.push(Command::Call(func_name, args.len() as u16 + 1));
            }
        };

        commands
    }
}

impl DumpVm for DoStatement {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let mut commands = self.0.dump_as_vm(context);
        commands.push(Command::Pop(Segment::Temp, 0));
        commands
    }
}

impl DumpVm for ReturnStatement {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let mut commands = vec![];

        for symbol in context.get_local_variables().iter().filter(|s| {
            s.kind == SymbolKind::Var && s.type_ == SymbolType::Class("String".to_string())
        }) {
            commands.extend(vec![
                Command::Push(Segment::Local, symbol.index as u16),
                Command::Call("String.dispose".to_string(), 1),
            ]);
        }

        if let Some(ref expression) = self.0.deref().item {
            commands.extend(expression.dump_as_vm(context));
        } else {
            commands.extend(vec![Command::Push(Segment::Constant, 0)]);
        };
        commands.push(Command::Return);
        commands
    }
}

impl DumpVm for Statement {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        match self {
            Statement::Let(let_) => let_.dump_as_vm(context),
            Statement::If(if_) => if_.dump_as_vm(context),
            Statement::While(while_) => while_.dump_as_vm(context),
            Statement::Do(do_) => do_.dump_as_vm(context),
            Statement::Return(return_) => return_.dump_as_vm(context),
        }
    }
}

impl DumpVm for Statements {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        self.statements.dump_as_vm(context)
    }
}

impl DumpVm for SubroutineBody {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let mut commands = vec![];

        for var_dec in self.var_decs.iter() {
            commands.extend(var_dec.dump_as_vm(context));
        }
        let body_commands = self.statements.dump_as_vm(context);

        let k = context.get_local_variable_count();
        let function_name = context.get_function_name().unwrap();
        commands.push(Command::Function(function_name, k as u16));
        for _ in 0..k {
            commands.push(Command::Push(Segment::Constant, 0));
        }

        let kind = context.get_function_kind().unwrap();
        if kind == SubroutineKind::Method {
            commands.extend(vec![
                Command::Push(Segment::Argument, 0),
                Command::Pop(Segment::Pointer, 0),
            ]);
        }
        if kind == SubroutineKind::Constructor {
            let k = context.get_instance_variable_count();
            commands.extend(vec![
                Command::Push(Segment::Constant, k as u16),
                Command::Call("Memory.alloc".to_string(), 1),
                Command::Pop(Segment::Pointer, 0),
            ]);
        }

        for symbol in context.get_local_variables().iter().filter(|s| {
            s.kind == SymbolKind::Var && s.type_ == SymbolType::Class("String".to_string())
        }) {
            let value = &symbol.name;
            commands.push(Command::Push(Segment::Constant, value.len() as u16));
            commands.push(Command::Call("String.new".to_string(), 1));
            for c in value.chars() {
                commands.push(Command::Push(Segment::Constant, c as u16));
                commands.push(Command::Call("String.appendChar".to_string(), 2));
            }
            commands.push(Command::Pop(Segment::Local, symbol.index as u16));
        }

        commands.extend(body_commands);
        commands
    }
}

impl DumpVm for ClassVarDec {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let type_: SymbolType = (*self.type_).clone().into();
        let kind = (*self.kind).into();
        self.var_names.iter().for_each(|identifier| {
            let Identifier(ref name) = **identifier;
            context.define_class_variable(name.clone(), type_.clone(), kind);
        });
        vec![]
    }
}

impl DumpVm for SubroutineDec {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let kind = *self.kind;
        let Identifier(ref name) = *self.subroutine_name;

        context.start_subroutine(name.clone(), kind);
        if kind == SubroutineKind::Method {
            context.define_method_variable(
                "this".to_string(),
                SymbolType::Class(context.class_name.clone()),
                SymbolKind::Argument,
            );
        }

        let mut commands = vec![];
        commands.extend(self.params.dump_as_vm(context));
        commands.extend(self.body.dump_as_vm(context));
        commands
    }
}

impl DumpVm for Class {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let mut commands = Vec::new();

        for var_dec in self.var_decs.iter() {
            commands.extend(var_dec.dump_as_vm(context));
        }

        for subroutine_dec in self.subroutine_decs.iter() {
            commands.extend(subroutine_dec.dump_as_vm(context));
        }

        commands
    }
}

impl<T: Parsable + DumpVm> DumpVm for Collection<T> {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        self.items
            .iter()
            .flat_map(|item| item.dump_as_vm(context))
            .collect()
    }
}
