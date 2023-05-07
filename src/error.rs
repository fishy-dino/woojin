use std::error::Error;

#[derive(Debug, Copy, Clone)]
pub enum WoojinErrorKind {
  Success,
  Unknown
}

#[derive(Debug)]
pub struct WoojinError {
  pub details: String,
  pub kind: WoojinErrorKind
}

impl WoojinError {
  pub fn new(msg: impl ToString) -> WoojinError {
    WoojinError{
      details: msg.to_string(),
      kind: WoojinErrorKind::Unknown
    }
  }
}

impl Error for WoojinError {}

impl std::fmt::Display for WoojinError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f,"WJ{}: {}", self.kind as i32, self.details)
  }
}

impl<T> From<nom::Err<T>> for WoojinError where T: std::fmt::Debug {
  fn from(err: nom::Err<T>) -> Self {
    WoojinError::new(format!("parse error: {}", err))
  }
}