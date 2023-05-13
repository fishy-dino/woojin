use std::error::Error;

#[derive(Debug, Copy, Clone)]
pub enum WoojinErrorKind {
  Roar,
  Success,
  Unknown,
  UnknownToken,
  FileNotFound,
  UnsupportedExtension,
  FailReadFailure,
  UndeclaredVariable,
  VariableAlreadyDeclared,
  VariableNotMutable,
  ParseError,
  CannotAdd,
  CannotSubtract,
  CannotMultiply,
  CannotDivide,
  DivisionByZero,
  InvalidType,
  TypeMismatch
}

#[derive(Debug)]
pub struct WoojinError {
  pub details: String,
  pub kind: WoojinErrorKind
}

impl WoojinError {
  pub fn new(msg: impl ToString, kind: WoojinErrorKind) -> WoojinError {
    WoojinError{
      details: msg.to_string(),
      kind
    }
  }

  pub fn exit(&self) -> ! {
    println!("{}", self);
    std::process::exit(1);
  }
}

impl Error for WoojinError {}

impl std::fmt::Display for WoojinError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f,"\x1b[1m\x1b[31mWJ{}\x1b[0m: {}", self.kind as i32, self.details)
  }
}

impl<T> From<nom::Err<T>> for WoojinError where T: std::fmt::Debug {
  fn from(err: nom::Err<T>) -> Self {
    WoojinError::new(format!("parse error: {}", err), WoojinErrorKind::ParseError)
  }
}