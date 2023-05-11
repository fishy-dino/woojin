use crate::variable::get_var;
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

impl WoojinValue {
  pub(crate) fn value(&self) -> WoojinValue {
    match self {
      WoojinValue::Var(name) => { get_var(name.as_str()).value.clone() },
      _ => self.clone(),
    }
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
