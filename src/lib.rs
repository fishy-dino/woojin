pub mod error;
pub mod token;
pub mod value;

use lazy_static::lazy_static;
use token::{Calc, WoojinResult};
use std::{
  process,
  io::{Write}, sync::{Arc, Mutex}, collections::HashMap,
};

use nom::{
  IResult
};
use value::{WoojinValue, ValueCalc};

#[derive(Debug)]
pub enum Statements {
  Nop,
  Comment(String),
  Input(Box<Statements>),
  Value(WoojinValue),
  Print(Vec<Box<Statements>>),
  Println(Vec<Box<Statements>>),
  Roar(WoojinValue),
  DecVar(String, Box<Statements>, bool),
  Calc(Calc),
  Sleep(Box<Statements>),
  Exit(i32)
}

#[derive(Debug)]
pub struct WoojinVariable {
  pub value: WoojinValue,
  pub is_mut: bool
}

pub type NomResult<'a, T> = IResult<&'a str, T>;
lazy_static!{
  pub static ref VARS: Arc<Mutex<HashMap<String, WoojinVariable>>> = {
    Arc::new(Mutex::new(HashMap::new()))
  };
}

pub fn exec(stmt: &Statements) -> Result<WoojinValue, crate::error::WoojinError> {
  match stmt {
    Statements::Exit(num) => { std::process::exit(*num); },
    Statements::Print(values) => {
      for (i, value) in values.iter().enumerate() {
        print!("{}", exec(value)?.to_print());
        if i != values.len() - 1 { print!(" "); }
      }
      std::io::stdout().flush().unwrap();
    },
    Statements::Println(values) => {
      for (i, value) in values.iter().enumerate() {
        print!("{}", exec(value)?.to_print());
        if i != values.len() - 1 { print!(" "); } else { print!("\n"); }
      }
      std::io::stdout().flush().unwrap();
    },
    Statements::Input(val) => {
      let mut input = String::new();
      exec(&Statements::Print(vec![Box::new(Statements::Value(exec(val.clone())?))]))?;
      std::io::stdin().read_line(&mut input).unwrap();
      return Ok(WoojinValue::String(input.trim().to_string()));
    },
    Statements::DecVar(name, value, is_mut) => {
      let mut vars = VARS.lock().unwrap();
      if vars.contains_key(name) { error(&format!("변수 {}가 이미 선언되었습니다", name)); }
      vars.insert(name.clone(), WoojinVariable { value: exec(&value)?, is_mut: *is_mut });
    },
    Statements::Calc(calc) => { return check_calc(calc.clone()); },
    Statements::Roar(val) => error(&val.to_print()),
    Statements::Value(val) => return Ok(val.clone()),
    Statements::Sleep(val) => {
      match exec(&**val)? {
        WoojinValue::Int(num) => std::thread::sleep(std::time::Duration::from_millis(num as u64)),
        _ => error(&"The param of the sleep function must be an integer")
      }
    },
    Statements::Nop => {}
    Statements::Comment(_) => {}
  }
  Ok(WoojinValue::Unit)
}

pub fn error(msg: &impl ToString) -> ! {
  eprintln!("Error: {}", msg.to_string());
  process::exit(1);
}

pub fn check_calc(calc: Calc) -> WoojinResult<WoojinValue> {
  match calc {
    Calc::Add(a, b) => Ok(check_calc(*a)?.add(&check_calc(*b)?)?),
    Calc::Sub(a, b) => Ok(check_calc(*a)?.sub(&check_calc(*b)?)?), // 빼기 연산
    Calc::Mul(a, b) => Ok(check_calc(*a)?.mul(&check_calc(*b)?)?), // 곱하기 연산
    Calc::Div(a, b) => Ok(check_calc(*a)?.div(&check_calc(*b)?)?), // 나누기 연산
    Calc::Value(val) => return Ok(val.clone()),
  }
}