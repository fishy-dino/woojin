use nom::{
  IResult,
  branch::{alt},
  multi::{fold_many0},
  bytes::complete::{ tag },
  character::complete::{ char, space0, digit1 },
  sequence::{pair, delimited},
  combinator::{map}
};
use crate::{types::WoojinValue, parser::WoojinResult, error::WoojinError};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Calc {
  Value(WoojinValue),
  Add(Box<Calc>, Box<Calc>),
  Sub(Box<Calc>, Box<Calc>),
  Mul(Box<Calc>, Box<Calc>),
  Div(Box<Calc>, Box<Calc>),
}

pub(crate) fn parse_primary(input: &str) -> IResult<&str, Calc> {
  alt((
    map(digit1, |digits: &str| {
      let value = digits.parse().unwrap();
      let woojin_value = WoojinValue::Int(value);
      Calc::Value(woojin_value)
    }),
    delimited(
      char('('),
      delimited(space0, parse_expr, space0),
      char(')'),
    ),
  ))(input.trim())
}

pub(crate) fn parse_expr(input: &str) -> IResult<&str, Calc> {
  let (input, init): (&str, Calc) = parse_term(input)?;
  let (input, pairs) = fold_many0(
    pair(alt((tag("+"), tag("-"))), parse_term),
    move || init.clone(),
    |acc, (op, val)| match op {
      "+" => Calc::Add(Box::new(acc), Box::new(val)),
      "-" => Calc::Sub(Box::new(acc), Box::new(val)),
      _ => unreachable!(),
    },
  )(input.trim())?;
  Ok((input.trim(), pairs))
}

pub(crate) fn parse_term(input: &str) -> IResult<&str, Calc> {
  let (input, init): (&str, Calc) = parse_primary(input)?;
  let (input, pairs) = fold_many0(
    pair(alt((tag("*"), tag("/"))), parse_primary),
    move || init.clone(),
    |acc, (op, val)| match op {
      "*" => Calc::Mul(Box::new(acc), Box::new(val)),
      "/" => Calc::Div(Box::new(acc), Box::new(val)),
      _ => unreachable!(),
    },
  )(input.trim())?;
  Ok((input.trim(), pairs))
}


pub(crate) fn parse_calc(input: &str) -> IResult<&str, Calc> {
  delimited(space0, parse_expr, space0)(input)
}

pub(crate) trait ValueCalc {
  fn add(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn sub(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn mul(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn div(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
}

impl ValueCalc for WoojinValue {
  fn add(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self, other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Int(a + b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float(a + b)),
      (WoojinValue::String(a), WoojinValue::String(b)) => Ok(WoojinValue::String(format!("{}{}", a, b))),
      _ => Err(WoojinError::new("The type that can't be added", crate::error::WoojinErrorKind::CannotAdd)),
    }  
  }

  fn sub(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self, other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Int(a - b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float(a - b)),
      _ => Err(WoojinError::new("The type that can't be subtracted", crate::error::WoojinErrorKind::CannotSubtract)),
    }  
  }

  fn mul(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self, other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Int(a * b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float(a * b)),
      _ => Err(WoojinError::new("The type that can't be multiplied!", crate::error::WoojinErrorKind::CannotMultiply)),
    }
  }

  fn div(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self, other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => {
        Ok(if b != 0 { WoojinValue::Int(a / b) } else {
          return Err(WoojinError::new("It cannot be divided by 0", crate::error::WoojinErrorKind::DivisionByZero))
        })
      },
      (WoojinValue::Float(a), WoojinValue::Float(b)) => {
        Ok(if b != 0.0 { WoojinValue::Float(a / b) } else {
          return Err(WoojinError::new("It cannot be divided by 0.0", crate::error::WoojinErrorKind::DivisionByZero))
        })
      },
      _ => Err(WoojinError::new("an indivisible type!", crate::error::WoojinErrorKind::CannotDivide)),
    }
  }
}