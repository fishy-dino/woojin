pub mod error;
pub mod types;
pub(crate) mod variable;
pub(crate) mod ast;
pub(crate) mod parser;
pub(crate) mod calc;

use std::{io::{Write}};
use ast::Statements;
use calc::{ValueCalc, Calc};
use error::WoojinError;
use nom::IResult;
use parser::{WoojinResult, tokenizer};
use types::WoojinValue;
use variable::WoojinVariable;

pub(crate) type NomResult<'a, T> = IResult<&'a str, T>;
// pub(crate) type StdString = std::string::String;

#[allow(dead_code)]
pub(crate) struct Program {
  pub(crate) pointer: i32,
  pub(crate) variables: Vec<WoojinVariable>,
  pub(crate) statements: Vec<Statements>
}

impl Program {
  pub fn new() -> Program {
    Program {
      pointer: 0,
      variables: Vec::new(),
      statements: Vec::new()
    }
  }
}

pub fn run(value: Vec<(usize, String)>) {
  let mut program: Program = Program::new();
  let lines: Vec<(usize, String)> = value.iter().map(|(a,b)| (*a, b.to_string())).collect();
  program.statements = match tokenizer(&lines) {
    Ok(statements) => statements,
    Err(e) => { e.exit(); }
  };

  run_program(&mut program);
}

pub(crate) fn run_program(program: &mut Program) {
  for stmt in &program.statements {
    match exec(stmt) {
      Ok(_) => {},
      Err(e) => { e.exit(); }
    }
  }
}

pub(crate) fn check_calc(calc: Calc) -> WoojinResult<WoojinValue> {
  match calc {
    Calc::Add(a, b) => Ok(check_calc(*a)?.add(&check_calc(*b)?)?),
    Calc::Sub(a, b) => Ok(check_calc(*a)?.sub(&check_calc(*b)?)?),
    Calc::Mul(a, b) => Ok(check_calc(*a)?.mul(&check_calc(*b)?)?),
    Calc::Div(a, b) => Ok(check_calc(*a)?.div(&check_calc(*b)?)?),
    Calc::Value(val) => return Ok(val.clone()),
  }
}

pub(crate) fn exec(stmt: &Statements) -> Result<WoojinValue, crate::error::WoojinError> {
  match stmt {
    Statements::Yee { code } => { std::process::exit(*code); },
    Statements::Roar { value } => { WoojinError::new(&value.to_print(), error::WoojinErrorKind::Roar); },
    Statements::Print { values } => {
      for (i, value) in values.iter().enumerate() {
        print!("{}", exec(value)?.to_print());
        if i != values.len() - 1 { print!(" "); }
      }
      std::io::stdout().flush().unwrap();
    },
    Statements::Println { values } => {
      for (i, value) in values.iter().enumerate() {
        print!("{}", exec(value)?.to_print());
        if i != values.len() - 1 { print!(" "); } else { print!("\n"); }
      }
      std::io::stdout().flush().unwrap();
    },
    Statements::Input { question } => {
      let mut input: String = String::new();
      if let Err(e) = exec(&Statements::Print{ values: vec![question.clone()] }) {
        e.exit();
      }
      std::io::stdin().read_line(&mut input).unwrap();
      return Ok(WoojinValue::String(input.trim().to_string()));
    },
    Statements::Sleep { value } => {
      match exec(&**value)? {
        WoojinValue::Int(num) => std::thread::sleep(std::time::Duration::from_millis(num as u64)),
        _ => WoojinError::new("The param of the sleep function must be an integer", error::WoojinErrorKind::TypeMismatch).exit()
      }
    },
    Statements::Assignment { name, value } => {
      let value: WoojinValue = exec(value)?;
      variable::change_var(name.as_str(), &value)?;
    },
    Statements::Let { name, stmt, kind, option } => { 
      let value: WoojinValue = exec(stmt)?;
      if !value.type_eq(*kind) { return Err(WoojinError::new("The type of the value and the type of the variable are different", error::WoojinErrorKind::TypeMismatch)); }
      variable::dec_var(name.as_str(), &value, option)?;
    },
    // Statements::If { condition: _, body: _ } => {}, 
    Statements::Value { value } => {
      return Ok(value.value().clone())
    },
    Statements::If { condition, stmt, else_stmt } => {
      match exec(condition)? {
        WoojinValue::Bool(b) => {
          for s in if b { stmt } else { else_stmt } { exec(s)?; }
        },
        _ => { return Err(WoojinError::new("The condition of the if statement must be a boolean", error::WoojinErrorKind::TypeMismatch)); }
      }
    },
    Statements::Calc(calc) => { return check_calc(calc.clone()); },
    Statements::Comment(_) => {}
  }
  Ok(WoojinValue::Unit)
}