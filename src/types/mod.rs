use std::str::FromStr;

use crate::{variable::get_var, error::WoojinError};
pub(crate) mod parse;

pub(crate) trait ToWoojinValue {
  fn to_woojin_value(&self) -> WoojinValue;
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum WoojinValue {
  Bool(bool),
  String(String),
  Int(i32),
  Long(i64), // Not yet supported
  Float(f32), // Not yet supported
  Double(f64), // Not yet supported
  Array(Vec<WoojinValue>), // Not yet supported
  Var(String),
  Unit,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum WoojinValueKind {
  Bool,
  String,
  Int,
  Long,
  Float,
  Double,
  Array,
  Unit,
  Any
}

impl WoojinValue {
  pub(crate) fn value(&self) -> WoojinValue {
    match self {
      WoojinValue::Var(name) => { get_var(name.as_str()).value.clone() },
      _ => self.clone(),
    }
  }

  pub(crate) fn kind(&self) -> WoojinValueKind {
    match self {
      WoojinValue::Bool(_) => WoojinValueKind::Bool,
      WoojinValue::String(_) => WoojinValueKind::String,
      WoojinValue::Int(_) => WoojinValueKind::Int,
      WoojinValue::Long(_) => WoojinValueKind::Long,
      WoojinValue::Float(_) => WoojinValueKind::Float,
      WoojinValue::Double(_) => WoojinValueKind::Double,
      WoojinValue::Array(_) => WoojinValueKind::Array,
      WoojinValue::Var(name) => get_var(name.as_str()).value.kind(),
      WoojinValue::Unit => WoojinValueKind::Unit
    }
  }

  pub(crate) fn type_eq(&self, other: WoojinValueKind) -> bool {
    if other == WoojinValueKind::Any { return true; }
    self.kind() == other
  }

  pub(crate) fn to_print(&self) -> String {
    match self {
      WoojinValue::Bool(a) => if a.clone() { String::from("uglyguri") } else { String::from("beautifulguri") },
      WoojinValue::Int(a) => a.to_string(),
      WoojinValue::Long(a) => a.to_string(),
      WoojinValue::Float(a) => a.to_string(),
      WoojinValue::Double(a) => a.to_string(),
      WoojinValue::String(a) => a.to_string(),
      WoojinValue::Array(a) => format!("[{}]", a.iter().map(|a| a.to_print()).collect::<Vec<String>>().join(", ")),
      WoojinValue::Var(name) => get_var(name.as_str()).value.to_print(),
      WoojinValue::Unit => "()".to_string()
    }
  }
}

impl FromStr for WoojinValueKind {
  type Err = WoojinError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "bool" => Ok(WoojinValueKind::Bool),
      "string" => Ok(WoojinValueKind::String),
      "int" => Ok(WoojinValueKind::Int),
      "long" => Ok(WoojinValueKind::Long),
      "float" => Ok(WoojinValueKind::Float),
      "double" => Ok(WoojinValueKind::Double),
      "array" => Ok(WoojinValueKind::Array),
      "unit" => Ok(WoojinValueKind::Unit),
      _ => Err(WoojinError::new(format!("Invalid type: {}", s), crate::error::WoojinErrorKind::InvalidType))
    }
  }
}

impl ToString for WoojinValueKind {
  fn to_string(&self) -> String {
    match self {
      WoojinValueKind::Bool => String::from("bool"),
      WoojinValueKind::String => String::from("string"),
      WoojinValueKind::Int => String::from("int"),
      WoojinValueKind::Long => String::from("long"),
      WoojinValueKind::Float => String::from("float"),
      WoojinValueKind::Double => String::from("double"),
      WoojinValueKind::Array => String::from("array"),
      WoojinValueKind::Unit => String::from("unit"),
      WoojinValueKind::Any => String::from("any")
    }
  }
}