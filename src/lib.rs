pub mod error;
pub mod types;
pub(crate) mod variable;
pub(crate) mod ast;
pub(crate) mod parser;
pub(crate) mod calc;

use std::io::{Write};
use ast::Statements;
use calc::{ValueCalc, Calc};
use error::WoojinError;
use nom::IResult;
use parser::{WoojinResult};
use types::WoojinValue;

pub(crate) type NomResult<'a, T> = IResult<&'a str, T>;
// pub(crate) type StdString = std::string::String;

pub(crate) struct Program { statements: Vec<self::ast::Statements> }
impl Program { pub fn new() -> Program { Program{ statements: Vec::new() } } }
pub fn run(lines: Vec<String>) {
  let mut program: Program = Program::new();
  for line in lines {
    let res: Result<Statements, error::WoojinError> = crate::parser::tokenizer(&line);
    match res {
      Ok(v) => {program.statements.push(v)},
      Err(err) => { err.exit(); }
    }
  }
  for stmt in program.statements.iter() { if let Err(err) = exec(stmt) { err.exit(); } }
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
        _ => WoojinError::new("The param of the sleep function must be an integer", error::WoojinErrorKind::Unknown).exit()
      }
    },
    Statements::Assignment { name, value } => {
      let value: WoojinValue = exec(value)?;
      variable::change_var(name.as_str(), &value)?;
    },
    Statements::Let { name, stmt, kind, option } => { 
      let value: WoojinValue = exec(stmt)?;
      if !value.type_eq(*kind) { return Err(WoojinError::new("The type of the value and the type of the variable are different", error::WoojinErrorKind::Unknown)); }
      variable::dec_var(name.as_str(), &value, option)?;
    },
    // Statements::If { condition: _, body: _ } => {}, 
    Statements::Value { value } => {
      return Ok(value.value().clone())
    },
    Statements::Calc(calc) => { return check_calc(calc.clone()); },
    Statements::Comment(_) => {}
  }
  Ok(WoojinValue::Unit)
}