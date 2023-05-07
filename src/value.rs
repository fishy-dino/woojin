use nom::{
  branch::alt,
  bytes::complete::{
    tag, escaped, is_not
  },
  character::complete::{char, digit1},
  combinator::{map, map_res, opt, recognize},
  sequence::{delimited, pair},
  IResult,
};

use crate::{token::{parse_variable_name, WoojinResult}, VARS, error::{WoojinError}, error};

// 정수(부호 있는 정수 포함)
pub fn parse_int(input: &str) -> IResult<&str, i32> {
  map_res(
      recognize(pair(opt(alt((char('+'), char('-')))), digit1)),
      |s: &str| s.parse::<i32>(),
  )(input)
}

pub fn parse_string(input: &str) -> IResult<&str, String> {
    let re = regex::Regex::new(r"\\").unwrap();
    let result = delimited(
        char('"'),
        escaped(
            is_not("\\\""),
            '\\',
            alt((
                tag("\\"),
                tag("\""),
                tag("t"),
                tag("n"),
                tag("r"),
                tag("0"),
            )),
        ),
        char('"'),
    )(input)?;
    let unescaped = result.1.replace("\\t", "\t").replace("\\r", "\r").replace("\\n", "\n");
    Ok((result.0, re.replace_all(&unescaped, "").to_string()))
}

// Boolean
pub fn parse_bool(input: &str) -> IResult<&str, bool> {
  alt((
    map(tag("uglyguri"), |_| true),
    map(tag("beautifulguri"), |_| false),
  ))(input)
}

pub fn parse_value(input: &str) -> IResult<&str, WoojinValue> {
  alt((
    map(parse_string, WoojinValue::String),
    map(parse_int, WoojinValue::Int),
    map(parse_bool, WoojinValue::Bool),
    map(parse_variable_name, WoojinValue::Var),
    // map(parse_array, WoojinValue::Array),
  ))(input)
}

pub(crate) trait ValueCalc {
  fn add(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn sub(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn mul(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn div(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum WoojinValue {
  Bool(bool),
  Int(i32),
  Long(i64),
  Float(f32),
  String(String),
  Array(Vec<WoojinValue>),
  Var(String),
  // Object(HashMap<WoojinValue, WoojinValue>),
  Unit
}

pub fn get_var(name: &str) -> WoojinValue {
  let vars = VARS.lock().unwrap();
  if !vars.contains_key(name) { error(&format!("Variable {} is not declared", name)); }
  vars.get(name).unwrap().value.clone()
}

impl WoojinValue {
  pub fn value(&self) -> WoojinValue {
    match self {
      WoojinValue::Var(name) => get_var(name.as_str()).clone(),
      _ => self.clone()
    }
  }

  pub fn to_print(&self) -> String {
    match self {
      WoojinValue::Bool(a) => if a.clone() { String::from("uglyguri") } else { String::from("beautifulguri") },
      WoojinValue::Int(a) => a.to_string(),
      WoojinValue::Long(a) => a.to_string(),
      WoojinValue::Float(a) => a.to_string(),
      // WoojinValue::Double(a) => a.to_string(),
      WoojinValue::String(a) => a.to_string(),
      WoojinValue::Array(a) => format!("[{}]", a.iter().map(|a| a.to_print()).collect::<Vec<String>>().join(", ")),
      WoojinValue::Var(name) => {
        get_var(name.as_str()).to_print()
      }
      // WoojinValue::Object(a) => a.iter().map(|(k, v)| format!("{}: {}", k.to_print(), v.to_print())).collect::<Vec<String>>().join(", "),
      WoojinValue::Unit => "()".to_string()
    }
  }
}

impl ValueCalc for WoojinValue {
  fn add(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self, other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Int(a + b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float(a + b)),
      (WoojinValue::String(a), WoojinValue::String(b)) => Ok(WoojinValue::String(format!("{}{}", a, b))),
      _ => Err(WoojinError::new("The type that can't be added")),
    }  
  }

  fn sub(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self, other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Int(a - b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float(a - b)),
      _ => Err(WoojinError::new("The type that can't be subtracted")),
    }  
  }

  fn mul(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self, other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Int(a * b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float(a * b)),
      _ => Err(WoojinError::new("The type that can't be multiplied!")),
    }
  }

  fn div(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self, other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => {
        Ok(if b != 0 { WoojinValue::Int(a / b) } else { return Err(WoojinError::new("It cannot be divided by 0")) })
      },
      (WoojinValue::Float(a), WoojinValue::Float(b)) => {
        Ok(if b != 0.0 { WoojinValue::Float(a / b) } else { return Err(WoojinError::new("It cannot be divided by 0.0")) })
      },
      _ => Err(WoojinError::new("an indivisible type!")),
  }
  
  }
}

pub struct Variable {
  pub mutability: bool,
  pub value: WoojinValue
}