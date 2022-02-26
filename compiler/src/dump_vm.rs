use anyhow::Context as _;
use anyhow::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::{self, Write};

use std::ops::Deref;

use super::foundation::*;
use super::program::*;
use crate::dump_xml::DumpXml;
use crate::expression::*;
use crate::parser::parse;
use crate::statement::*;
use crate::tokenizer::tokenize;
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

impl DumpVm for Term {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        match self {
            Term::Constant(value) => match **value {
                Constant::Null => {
                    vec![Command::Push(Segment::Constant, 0)]
                }
                Constant::Integer(value) => {
                    vec![Command::Push(Segment::Constant, value as u16)]
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
                _ => panic!("unsupported constant type {:?}", value),
            },
            Term::Variable(identifier) => {
                let Identifier(ref name) = **identifier;
                let symbol = context.symbol_table.get(&name).unwrap();
                match symbol.kind {
                    SymbolKind::Argument => {
                        vec![Command::Push(Segment::Argument, symbol.index as u16)]
                    }
                    SymbolKind::Var => {
                        vec![Command::Push(Segment::Local, symbol.index as u16)]
                    }
                    SymbolKind::Field => {
                        vec![Command::Push(Segment::This, symbol.index as u16)]
                    }
                    SymbolKind::Static => {
                        vec![Command::Push(Segment::Static, symbol.index as u16)]
                    }
                }
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
                let segment = match attr.kind {
                    SymbolKind::Var => Segment::Local,
                    SymbolKind::Argument => Segment::Argument,
                    SymbolKind::Field => Segment::This,
                    SymbolKind::Static => Segment::Static,
                };

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
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
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
            commands.append(&mut term.dump_as_vm(context));
            commands.append(&mut op.dump_as_vm(context));
        }
        commands
    }
}

impl DumpVm for LetStatement {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let Identifier(ref name) = self.var_name.deref();
        let symbol = context.get(name).unwrap();
        let segment = match symbol.kind {
            SymbolKind::Argument => Segment::Argument,
            SymbolKind::Var => Segment::Local,
            SymbolKind::Field => Segment::This,
            SymbolKind::Static => Segment::Static,
        };

        let mut commands = vec![];

        // evaluate right hand
        let mut push_expression_result = self.expression.dump_as_vm(context);
        commands.extend(push_expression_result);

        if let Some(index_expression) = self.array_size.deref() {
            let mut push_array_size_result = index_expression.dump_as_vm(context);
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
                        let segment = match symbol.kind {
                            SymbolKind::Static => Segment::Static,
                            SymbolKind::Field => Segment::This,
                            SymbolKind::Argument => Segment::Argument,
                            SymbolKind::Var => Segment::Local,
                        };
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

        for symbol in context
            .get_local_variables()
            .iter()
            .filter(|s| s.kind == SymbolKind::Var && s.type_ == SymbolType::Class("String".to_string()))
        {
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
        let mut commands = Vec::new();

        for var_dec in self.var_decs.iter() {
            commands.append(&mut var_dec.dump_as_vm(context));
        }
        let mut body_commands = self.statements.dump_as_vm(context);

        let k = context.get_local_variable_count();
        let function_name = context.get_function_name().unwrap();
        commands.push(Command::Function(function_name, k as u16));
        for _ in 0..k {
            commands.push(Command::Push(Segment::Constant, 0));
        }

        let kind = context.get_function_kind().unwrap();
        if kind == SubroutineKind::Method {
            commands.push(Command::Push(Segment::Argument, 0));
            commands.push(Command::Pop(Segment::Pointer, 0));
        }
        if kind == SubroutineKind::Constructor {
            let k = context.get_instance_variable_count();
            commands.push(Command::Push(Segment::Constant, k as u16));
            commands.push(Command::Call("Memory.alloc".to_string(), 1));
            commands.push(Command::Pop(Segment::Pointer, 0));
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
        let type_ = (*self.type_).clone();
        let kind = *self.kind;
        let Identifier(ref name) = *self.subroutine_name;

        context.start_subroutine(name.clone(), kind);
        let mut commands = vec![];

        if kind == SubroutineKind::Method {
            context.define_method_variable(
                "this".to_string(),
                SymbolType::Class(context.class_name.clone()),
                SymbolKind::Argument,
            );
        }
        commands.append(&mut self.params.dump_as_vm(context));
        commands.append(&mut self.body.dump_as_vm(context));
        commands
    }
}

impl DumpVm for Class {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        let mut commands = Vec::new();

        for var_dec in self.var_decs.iter() {
            commands.append(&mut var_dec.dump_as_vm(context));
        }

        for subroutine_dec in self.subroutine_decs.iter() {
            commands.append(&mut subroutine_dec.dump_as_vm(context));
        }

        commands
    }
}

#[test]
fn test_p1() {
    let main = r#"
class Main {
    function void main() {
        var SquareGame game;
        let game = SquareGame.new();
        do game.run();
        do game.dispose();
        return;
    }
}
    "#;

    let square = r#"
class Square {

   field int x, y; // screen location of the square's top-left corner
   field int size; // length of this square, in pixels

   /** Constructs a new square with a given location and size. */
   constructor Square new(int Ax, int Ay, int Asize) {
      let x = Ax;
      let y = Ay;
      let size = Asize;
      do draw();
      return this;
   }

   /** Disposes this square. */
   method void dispose() {
      do Memory.deAlloc(this);
      return;
   }

   /** Draws the square on the screen. */
   method void draw() {
      do Screen.setColor(true);
      do Screen.drawRectangle(x, y, x + size, y + size);
      return;
   }

   /** Erases the square from the screen. */
   method void erase() {
      do Screen.setColor(false);
      do Screen.drawRectangle(x, y, x + size, y + size);
      return;
   }

    /** Increments the square size by 2 pixels. */
   method void incSize() {
      if (((y + size) < 254) & ((x + size) < 510)) {
         do erase();
         let size = size + 2;
         do draw();
      }
      return;
   }

   /** Decrements the square size by 2 pixels. */
   method void decSize() {
      if (size > 2) {
         do erase();
         let size = size - 2;
         do draw();
      }
      return;
   }

   /** Moves the square up by 2 pixels. */
   method void moveUp() {
      if (y > 1) {
         do Screen.setColor(false);
         do Screen.drawRectangle(x, (y + size) - 1, x + size, y + size);
         let y = y - 2;
         do Screen.setColor(true);
         do Screen.drawRectangle(x, y, x + size, y + 1);
      }
      return;
   }

   /** Moves the square down by 2 pixels. */
   method void moveDown() {
      if ((y + size) < 254) {
         do Screen.setColor(false);
         do Screen.drawRectangle(x, y, x + size, y + 1);
         let y = y + 2;
         do Screen.setColor(true);
         do Screen.drawRectangle(x, (y + size) - 1, x + size, y + size);
      }
      return;
   }

   /** Moves the square left by 2 pixels. */
   method void moveLeft() {
      if (x > 1) {
         do Screen.setColor(false);
         do Screen.drawRectangle((x + size) - 1, y, x + size, y + size);
         let x = x - 2;
         do Screen.setColor(true);
         do Screen.drawRectangle(x, y, x + 1, y + size);
      }
      return;
   }

   /** Moves the square right by 2 pixels. */
   method void moveRight() {
      if ((x + size) < 510) {
         do Screen.setColor(false);
         do Screen.drawRectangle(x, y, x + 1, y + size);
         let x = x + 2;
         do Screen.setColor(true);
         do Screen.drawRectangle((x + size) - 1, y, x + size, y + size);
      }
      return;
   }
}
    "#;

    let game = r#"
class SquareGame {
   field Square square; // the square of this game
   field int direction; // the square's current direction: 
                        // 0=none, 1=up, 2=down, 3=left, 4=right

   /** Constructs a new Square Game. */
   constructor SquareGame new() {
      // Creates a 30 by 30 pixels square and positions it at the top-left
      // of the screen.
      let square = Square.new(0, 0, 30);
      let direction = 0;  // initial state is no movement
      return this;
   }

   /** Disposes this game. */
   method void dispose() {
      do square.dispose();
      do Memory.deAlloc(this);
      return;
   }

   /** Moves the square in the current direction. */
   method void moveSquare() {
      if (direction = 1) { do square.moveUp(); }
      if (direction = 2) { do square.moveDown(); }
      if (direction = 3) { do square.moveLeft(); }
      if (direction = 4) { do square.moveRight(); }
      do Sys.wait(5);  // delays the next movement
      return;
   }

   /** Runs the game: handles the user's inputs and moves the square accordingly */
   method void run() {
      var char key;  // the key currently pressed by the user
      var boolean exit;
      let exit = false;
      
      while (~exit) {
         // waits for a key to be pressed
         while (key = 0) {
            let key = Keyboard.keyPressed();
            do moveSquare();
         }
         if (key = 81)  { let exit = true; }     // q key
         if (key = 90)  { do square.decSize(); } // z key
         if (key = 88)  { do square.incSize(); } // x key
         if (key = 131) { let direction = 1; }   // up arrow
         if (key = 133) { let direction = 2; }   // down arrow
         if (key = 130) { let direction = 3; }   // left arrow
         if (key = 132) { let direction = 4; }   // right arrow

         // waits for the key to be released
         while (~(key = 0)) {
            let key = Keyboard.keyPressed();
            do moveSquare();
         }
     } // while
     return;
   }
}
    "#;

    fn compile(program: &str, name: String) -> Vec<Command> {
        let tokens = tokenize(program).unwrap();
        let class = parse(&tokens).unwrap();
        let mut file = File::create(format!(
            "/workspaces/nand2tetris-rs/compiler/tests/c/{}.vm",
            &name
        ))
        .unwrap();

        let mut context = Context::new(name);
        let commands = class.dump_as_vm(&mut context);
        for c in &commands {
            writeln!(file, "{}", c.dump()).unwrap();
        }
        file.flush().unwrap();

        commands
    }
    let main_commands = compile(main, "Main".to_string());

    let square_commands = compile(square, "Square".to_string());
    let game_commands = compile(game, "SquareGame".to_string());
    //for c in commands {
    //    println!("{}", c.dump());
    //}
}

//#[test]
//fn test_class() {
//    let mut context = Context::new("Test".to_string());
//    let class = Class {
//        class_name: Box::new(Identifier("Test".to_string())),
//        var_decs: Box::new(vec![
//            Box::new(ClassVarDec {
//                kind: Box::new(ClassVarKind::Static),
//                type_: Box::new(Type::Int),
//                var_names: Box::new(vec![Box::new(Identifier("a".to_string()))]),
//            }),
//            Box::new(ClassVarDec {
//                kind: Box::new(ClassVarKind::Static),
//                type_: Box::new(Type::Int),
//                var_names: Box::new(vec![Box::new(Identifier("b".to_string()))]),
//            }),
//            Box::new(ClassVarDec {
//                kind: Box::new(ClassVarKind::Field),
//                type_: Box::new(Type::Char),
//                var_names: Box::new(vec![Box::new(Identifier("c".to_string()))]),
//            }),
//        ]),
//        subroutine_decs: Box::new(vec![Box::new(SubroutineDec {
//            kind: Box::new(SubroutineKind::Method),
//            type_: Box::new(SubroutineType::Void),
//            subroutine_name: Box::new(Identifier("method".to_string())),
//            params: Box::new(ParameterList {
//                params: vec![Box::new((
//                    Box::new(Type::Char),
//                    Box::new(Identifier("c".to_string())),
//                ))],
//            }),
//            body: Box::new(SubroutineBody {
//                var_decs: Box::new(vec![Box::new(VarDec {
//                    type_: Box::new(Type::Int),
//                    var_names: Box::new(vec![Box::new(Identifier("a".to_string()))]),
//                })]),
//                statements: Box::new(Statements {
//                    statements: Box::new(Collection::<Statement>::new(vec![
//                        Box::new(Statement::Let(Box::new(LetStatement {
//                            var_name: Box::new(Identifier("a".to_string())),
//                            array_size: Box::new(None),
//                            expression: Box::new(Expression {
//                                term: Box::new(Term::Constant(Box::new(Constant::Integer(1)))),
//                                extras: Box::new(Collection::<Seq2<Op, Term>>::new(vec![])),
//                            }),
//                        }))),
//                        //Box::new(Statement::Let(LetStatement {
//                        //    var_name: Box::new(Identifier("b".to_string())),
//                        //    expression: Box::new(Expression::Term(Term::IntConstant(2))),
//                        //})),
//                        //Box::new(Statement::Let(LetStatement {
//                        //    var_name: Box::new(Identifier("c".to_string())),
//                        //    expression: Box::new(Expression::Term(Term::IntConstant(3))),
//                        //})),
//                        //Box::new(Statement::Return(ReturnStatement { expression: None })),
//                    ])),
//                }),
//            }),
//        })]),
//    };
//    let commands = class.dump_as_vm(&mut context);
//    dbg!(&context);
//    for c in commands {
//        println!("{}", c.dump());
//    }
//    //assert_eq!(commands.len(), 1);
//}

#[test]
fn test_p2() {
    let main = r#"
class Main {
   function void main() {
     var Array a; 
     var int length;
     var int i, sum;

     let length = Keyboard.readInt("How many numbers? ");
     let a = Array.new(length); // constructs the array
     
     let i = 0;
     while (i < length) {
        let a[i] = Keyboard.readInt("Enter a number: ");
        let sum = sum + a[i];
        let i = i + 1;
     }
     
     do Output.printString("The average is ");
     do Output.printInt(sum / length);
     return;
   }
}"#;

    fn compile(program: &str, name: String) -> Vec<Command> {
        let tokens = tokenize(program).unwrap();
        let class = parse(&tokens).unwrap();
        let mut file = File::create(format!(
            "/workspaces/nand2tetris-rs/compiler/tests/d/{}.vm",
            &name
        ))
        .unwrap();

        let mut context = Context::new(name);
        let commands = class.dump_as_vm(&mut context);
        for c in &commands {
            writeln!(file, "{}", c.dump()).unwrap();
        }
        file.flush().unwrap();

        commands
    }
    let main_commands = compile(main, "Main".to_string());
}

impl<T: Parsable + DumpVm> DumpVm for Collection<T> {
    fn dump_as_vm(&self, context: &mut Context) -> Vec<Command> {
        self.items
            .iter()
            .flat_map(|item| item.dump_as_vm(context))
            .collect()
    }
}

#[test]
fn test_p3() {
    let main = r#"
class Main {

    /** Initializes a Pong game and starts running it. */
    function void main() {
        var PongGame game;
        do PongGame.newInstance();
        let game = PongGame.getInstance();
        do game.run();
        do game.dispose();
        return;
    }
}"#;

    let ball = r#"
class Ball {

    field int x, y;               // the ball's screen location (in pixels)
    field int lengthx, lengthy;   // distance of last destination (in pixels)

    field int d, straightD, diagonalD;            // used for straight line movement computation
    field boolean invert, positivex, positivey;   // (same)
   
    field int leftWall, rightWall, topWall, bottomWall;  // wall locations
   
    field int wall;   // last wall that the ball was bounced off of

    /** Constructs a new ball with the given initial location and wall locations. */
    constructor Ball new(int Ax, int Ay,
                         int AleftWall, int ArightWall, int AtopWall, int AbottomWall) {    	
	    let x = Ax;		
	    let y = Ay;
	    let leftWall = AleftWall;
	    let rightWall = ArightWall - 6;    // -6 for ball size
	    let topWall = AtopWall; 
	    let bottomWall = AbottomWall - 6;  // -6 for ball size
	    let wall = 0;
        do show();
        return this;
    }

    /** Deallocates the Ball's memory. */
    method void dispose() {
        do Memory.deAlloc(this);
        return;
    }

    /** Shows the ball. */
    method void show() {
        do Screen.setColor(true);
        do draw();
        return;
    }

    /** Hides the ball. */
    method void hide() {
        do Screen.setColor(false);
	    do draw();
        return;
    }

    /** Draws the ball. */
    method void draw() {
	    do Screen.drawRectangle(x, y, x + 5, y + 5);
	    return;
    }

    /** Returns the ball's left edge. */
    method int getLeft() {
        return x;
    }

    /** Returns the ball's right edge. */
    method int getRight() {
        return x + 5;
    }

    /** Computes and sets the ball's destination. */
    method void setDestination(int destx, int desty) {
        var int dx, dy, temp;
  	    let lengthx = destx - x;
	    let lengthy = desty - y;
        let dx = Math.abs(lengthx);
        let dy = Math.abs(lengthy);
        let invert = (dx < dy);

        if (invert) {
            let temp = dx; // swap dx, dy
            let dx = dy;
            let dy = temp;
   	        let positivex = (y < desty);
            let positivey = (x < destx);
        }
        else {
	        let positivex = (x < destx);
            let positivey = (y < desty);
        }

        let d = (2 * dy) - dx;
        let straightD = 2 * dy;
        let diagonalD = 2 * (dy - dx);

	    return;
    }

    /**
     * Moves the ball one unit towards its destination.
     * If the ball has reached a wall, returns 0.
     * Else, returns a value according to the wall:
     * 1 (left wall), 2 (right wall), 3 (top wall), 4 (bottom wall).
     */
    method int move() {

	    do hide();

        if (d < 0) { let d = d + straightD; }
        else {
            let d = d + diagonalD;

            if (positivey) {
                if (invert) { let x = x + 4; }
                else { let y = y + 4; }
            }
            else {
                if (invert) { let x = x - 4; }
                else { let y = y - 4; }
            }
	    }

        if (positivex) {
            if (invert) { let y = y + 4; }
            else { let x = x + 4; }
	    }
	    else {
            if (invert) { let y = y - 4; }
            else { let x = x - 4; }
	    }

	    if (~(x > leftWall)) {
	        let wall = 1;    
	        let x = leftWall;
	    }
        if (~(x < rightWall)) {
	        let wall = 2;    
	        let x = rightWall;
	    }
        if (~(y > topWall)) {
            let wall = 3;    
	        let y = topWall;
        }
        if (~(y < bottomWall)) {
            let wall = 4;    
	        let y = bottomWall;
        }

	    do show();

	    return wall;
    }

    /**
     * Bounces off the current wall: sets the new destination
     * of the ball according to the ball's angle and the given
     * bouncing direction (-1/0/1=left/center/right or up/center/down).
     */
    method void bounce(int bouncingDirection) {
        var int newx, newy, divLengthx, divLengthy, factor;

	    // dividing by 10 first since results are too big
        let divLengthx = lengthx / 10;
        let divLengthy = lengthy / 10;
	    if (bouncingDirection = 0) { let factor = 10; }
	    else {
	        if (((~(lengthx < 0)) & (bouncingDirection = 1)) | ((lengthx < 0) & (bouncingDirection = (-1)))) {
                let factor = 20; // bounce direction is in ball direction
            }
	        else { let factor = 5; } // bounce direction is against ball direction
	    }

	    if (wall = 1) {
	        let newx = 506;
	        let newy = (divLengthy * (-50)) / divLengthx;
            let newy = y + (newy * factor);
	    }
        else {
            if (wall = 2) {
                let newx = 0;
                let newy = (divLengthy * 50) / divLengthx;
                let newy = y + (newy * factor);
	        }
	        else {
                if (wall = 3) {
		            let newy = 250;
		            let newx = (divLengthx * (-25)) / divLengthy;
                    let newx = x + (newx * factor);
		        }
                else { // assumes wall = 4
		            let newy = 0;
		            let newx = (divLengthx * 25) / divLengthy;
                    let newx = x + (newx * factor);
		        }
            }
        }

        do setDestination(newx, newy);
        return;
    }
}"#;

    let bat = r#"
class Bat {

    field int x, y;           // the bat's screen location
    field int width, height;  // the bat's width and height
    field int direction;      // direction of the bat's movement (1 = left, 2 = right)

    /** Constructs a new bat with the given location and width. */
    constructor Bat new(int Ax, int Ay, int Awidth, int Aheight) {
        let x = Ax;
        let y = Ay;
        let width = Awidth;
        let height = Aheight;
        let direction = 2;
        do show();
        return this;
    }

    /** Deallocates the object's memory. */
    method void dispose() {
        do Memory.deAlloc(this);
        return;
    }

    /** Shows the bat. */
    method void show() {
        do Screen.setColor(true);
        do draw();
        return;
    }

    /** Hides the bat. */
    method void hide() {
        do Screen.setColor(false);
        do draw();
        return;
    }

    /** Draws the bat. */
    method void draw() {
        do Screen.drawRectangle(x, y, x + width, y + height);
        return;
    }

    /** Sets the bat's direction (0=stop, 1=left, 2=right). */
    method void setDirection(int Adirection) {
        let direction = Adirection;
        return;
    }

    /** Returns the bat's left edge. */
    method int getLeft() {
        return x;
    }

    /** Returns the bat's right edge. */
    method int getRight() {
        return x + width;
    }

    /** Sets the bat's width. */
    method void setWidth(int Awidth) {
        do hide();
        let width = Awidth;
        do show();
        return;
    }

    /** Moves the bat one step in the bat's direction. */
    method void move() {
	    if (direction = 1) {
            let x = x - 4;
            if (x < 0) { let x = 0; }
            do Screen.setColor(false);
            do Screen.drawRectangle((x + width) + 1, y, (x + width) + 4, y + height);
            do Screen.setColor(true);
            do Screen.drawRectangle(x, y, x + 3, y + height);
        }
        else {
            let x = x + 4;
            if ((x + width) > 511) { let x = 511 - width; }
            do Screen.setColor(false);
            do Screen.drawRectangle(x - 4, y, x - 1, y + height);
            do Screen.setColor(true);
            do Screen.drawRectangle((x + width) - 3, y, x + width, y + height);
        }
        return;
    }
}"#;

    let pong_game = r#"
class PongGame {

    static PongGame instance; // the singelton, a Pong game instance     
    field Bat bat;            // the bat
    field Ball ball;          // the ball
    field int wall;           // the current wall that the ball is bouncing off of.
    field boolean exit;       // true when the game is over
    field int score;          // the current score.
    field int lastWall;       // the last wall that the ball bounced off of.

    // The current width of the bat
    field int batWidth;

    /** Constructs a new Pong game. */
    constructor PongGame new() {
	    do Screen.clearScreen();
        let batWidth = 50;  // initial bat size
        let bat = Bat.new(230, 229, batWidth, 7);
        let ball = Ball.new(253, 222, 0, 511, 0, 229);
        do ball.setDestination(400,0);
        do Screen.drawRectangle(0, 238, 511, 240);
	    do Output.moveCursor(22,0);
	    do Output.printString("Score: 0");
	
	    let exit = false;
	    let score = 0;
	    let wall = 0;
	    let lastWall = 0;

        return this;
    }

    /** Deallocates the object's memory. */
    method void dispose() {
        do bat.dispose();
	    do ball.dispose();
        do Memory.deAlloc(this);
        return;
    }

    /** Creates an instance of Pong game, and stores it. */
    function void newInstance() {
        let instance = PongGame.new();
        return;
    }
    
    /** Returns the single instance of this Pong game. */
    function PongGame getInstance() {
        return instance;
    }

    /** Starts the game, and andles inputs from the user that control
     *  the bat's movement direction. */
    method void run() {
        var char key;

        while (~exit) {
            // waits for a key to be pressed.
            while ((key = 0) & (~exit)) {
                let key = Keyboard.keyPressed();
                do bat.move();
                do moveBall();
                do Sys.wait(50);
            }

            if (key = 130) { do bat.setDirection(1); }
	        else {
	            if (key = 132) { do bat.setDirection(2); }
		        else {
                    if (key = 140) { let exit = true; }
		        }
            }

            // Waits for the key to be released.
            while ((~(key = 0)) & (~exit)) {
                let key = Keyboard.keyPressed();
                do bat.move();
                do moveBall();
                do Sys.wait(50);
            }
        }

	    if (exit) {
            do Output.moveCursor(10,27);
	        do Output.printString("Game Over");
	    }
            
        return;
    }

    /**
     * Handles ball movement, including bouncing.
     * If the ball bounces off a wall, finds its new direction.
     * If the ball bounces off the bat, increases the score by one
     * and shrinks the bat's size, to make the game more challenging. 
     */
    method void moveBall() {
        var int bouncingDirection, batLeft, batRight, ballLeft, ballRight;

        let wall = ball.move();

        if ((wall > 0) & (~(wall = lastWall))) {
            let lastWall = wall;
            let bouncingDirection = 0;
            let batLeft = bat.getLeft();
            let batRight = bat.getRight();
            let ballLeft = ball.getLeft();
            let ballRight = ball.getRight();
  
            if (wall = 4) {
                let exit = (batLeft > ballRight) | (batRight < ballLeft);
                if (~exit) {
                    if (ballRight < (batLeft + 10)) { let bouncingDirection = -1; }
                    else {
                        if (ballLeft > (batRight - 10)) { let bouncingDirection = 1; }
                    }

                    let batWidth = batWidth - 2;
                    do bat.setWidth(batWidth);      
                    let score = score + 1;
                    do Output.moveCursor(22,7);
                    do Output.printInt(score);
                }
            }
            do ball.bounce(bouncingDirection);
        }
        return;
    }
}"#;


    fn compile(program: &str, name: String){
        let tokens = tokenize(program).unwrap();
        let class = parse(&tokens).unwrap();
        let mut file = File::create(format!(
            "/workspaces/nand2tetris-rs/compiler/tests/e/{}.vm",
            &name
        ))
        .unwrap();

        let mut context = Context::new(name);
        let commands = class.dump_as_vm(&mut context);
        for c in &commands {
            writeln!(file, "{}", c.dump()).unwrap();
        }
        file.flush().unwrap();
    }
    compile(main, "Main".to_string());
    compile(ball, "Ball".to_string());
    compile(bat, "Bat".to_string());
    compile(pong_game, "PongGame".to_string());
}
#[test]
fn test_p4() {
    let main = r#"
class Main {

    function void main() {
        var Array a, b, c;
        
        let a = Array.new(10);
        let b = Array.new(5);
        let c = Array.new(1);
        
        let a[3] = 2;
        let a[4] = 8;
        let a[5] = 4;
        let b[a[3]] = a[3] + 3;  // b[2] = 5
        let a[b[a[3]]] = a[a[5]] * b[((7 - a[3]) - Main.double(2)) + 1];  // a[5] = 8 * 5 = 40
        let c[0] = null;
        let c = c[0];
        
        do Output.printString("Test 1: expected result: 5; actual result: ");
        do Output.printInt(b[2]);
        do Output.println();
        do Output.printString("Test 2: expected result: 40; actual result: ");
        do Output.printInt(a[5]);
        do Output.println();
        do Output.printString("Test 3: expected result: 0; actual result: ");
        do Output.printInt(c);
        do Output.println();
        
        let c = null;

        if (c = null) {
            do Main.fill(a, 10);
            let c = a[3];
            let c[1] = 33;
            let c = a[7];
            let c[1] = 77;
            let b = a[3];
            let b[1] = b[1] + c[1];  // b[1] = 33 + 77 = 110;
        }

        do Output.printString("Test 4: expected result: 77; actual result: ");
        do Output.printInt(c[1]);
        do Output.println();
        do Output.printString("Test 5: expected result: 110; actual result: ");
        do Output.printInt(b[1]);
        do Output.println();
        return;
    }
    
    function int double(int a) {
    	return a * 2;
    }
    
    function void fill(Array a, int size) {
        while (size > 0) {
            let size = size - 1;
            let a[size] = Array.new(3);
        }
        return;
    }
}"#;

    fn compile(program: &str, name: String){
        let tokens = tokenize(program).unwrap();
        let class = parse(&tokens).unwrap();
        let mut file = File::create(format!(
            "/workspaces/nand2tetris-rs/compiler/tests/f/{}.vm",
            &name
        ))
        .unwrap();

        let mut context = Context::new(name);
        let commands = class.dump_as_vm(&mut context);
        for c in &commands {
            writeln!(file, "{}", c.dump()).unwrap();
        }
        file.flush().unwrap();
    }
    compile(main, "Main".to_string());

}