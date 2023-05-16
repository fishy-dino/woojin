use nom::{
  IResult,
  branch::{alt},
  multi::{fold_many0},
  bytes::complete::{ tag },
  character::complete::{ char, space0 },
  sequence::{pair, delimited},
  combinator::{map}
};
use crate::{types::{WoojinValue, parse::parse_value}, parser::{WoojinResult}, error::{WoojinError}};

#[derive(Debug, Clone)]
pub(crate) enum Calc {
  Value(WoojinValue),
  Add(Box<Calc>, Box<Calc>),
  Sub(Box<Calc>, Box<Calc>),
  Mul(Box<Calc>, Box<Calc>),
  Div(Box<Calc>, Box<Calc>),
  Equal(Box<Calc>, Box<Calc>),
  NotEqual(Box<Calc>, Box<Calc>),
  GreaterThan(Box<Calc>, Box<Calc>),
  LessThan(Box<Calc>, Box<Calc>),
  GreaterThanOrEqual(Box<Calc>, Box<Calc>),
  LessThanOrEqual(Box<Calc>, Box<Calc>),
}

pub(crate) fn parse_primary(input: &str) -> IResult<&str, Calc> {
  alt((
      map(parse_value, |value| Calc::Value(value)),
      delimited(
          char('('),
          delimited(space0, parse_expr, space0),
          char(')'),
      )
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
  let (input, init): (&str, Calc) = parse_factor(input)?;
  let (input, pairs) = fold_many0(
      pair(alt((tag("*"), tag("/"))), parse_factor),
      move || init.clone(),
      |acc, (op, val)| match op {
          "*" => Calc::Mul(Box::new(acc), Box::new(val)),
          "/" => Calc::Div(Box::new(acc), Box::new(val)),
          _ => unreachable!(),
      },
  )(input.trim())?;
  Ok((input.trim(), pairs))
}

pub(crate) fn parse_factor(input: &str) -> IResult<&str, Calc> {
  let (input, init): (&str, Calc) = parse_comparison(input)?;
  let (input, pairs) = fold_many0(
      pair(
          alt((
              tag("="),
              tag("!="),
          )),
          alt((
            parse_primary,
          )),
      ),
      move || init.clone(),
      |acc, (op, val)| match op {
          "=" => Calc::Equal(Box::new(acc), Box::new(val)),
          "!=" => Calc::NotEqual(Box::new(acc), Box::new(val)),
          _ => unreachable!(),
      },
  )(input.trim())?;

  Ok((input.trim(), pairs))
}

pub(crate) fn parse_comparison(input: &str) -> IResult<&str, Calc> {
  let (input, init) = parse_primary(input)?;
  let (input, pairs) = fold_many0(
      pair(
          alt((
              tag("=="),
              tag("!="),
              tag("="),
              tag(">="),
              tag("<="),
              tag("<"),
              tag(">"),
          )),
          parse_primary,
      ),
      || init.clone(),
      |acc, (op, val)| match op {
          "==" => Calc::Equal(Box::new(acc), Box::new(val)),
          "!=" => Calc::NotEqual(Box::new(acc), Box::new(val)),
          ">=" => Calc::GreaterThanOrEqual(Box::new(acc), Box::new(val)),
          "<=" => Calc::LessThanOrEqual(Box::new(acc), Box::new(val)),
          "<" => Calc::LessThan(Box::new(acc), Box::new(val)),
          ">" => Calc::GreaterThan(Box::new(acc), Box::new(val)),
          _ => unreachable!(),
      },
  )(input.trim())?;

  Ok((input, pairs))
}

pub(crate) fn parse_calc(input: &str) -> IResult<&str, Calc> {
  delimited(space0, parse_expr, space0)(input)
}

pub(crate) trait ValueCalc {
  fn equal(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn not_equal(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn biggerthen(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn smallerthen(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn biggerthen_equal(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn smallerthen_equal(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn add(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn sub(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn mul(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
  fn div(&self, other: &WoojinValue) -> WoojinResult<WoojinValue>;
}

impl ValueCalc for WoojinValue {
  fn equal(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self.value(), other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Bool(a == b)),
      (WoojinValue::Long(a), WoojinValue::Long(b)) => Ok(WoojinValue::Bool(a == b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Bool(a == b)),
      (WoojinValue::Double(a), WoojinValue::Double(b)) => Ok(WoojinValue::Bool(a == b)),
      (WoojinValue::String(a), WoojinValue::String(b)) => Ok(WoojinValue::Bool(a == b)),
      (WoojinValue::Bool(a), WoojinValue::Bool(b)) => Ok(WoojinValue::Bool(a == b)),
      _ => Err(WoojinError::new("The type that can't be compared", crate::error::WoojinErrorKind::CannotCompare)),
    }  
  }

  fn not_equal(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match self.equal(other) {
      Ok(WoojinValue::Bool(a)) => Ok(WoojinValue::Bool(!a)),
      _ => Err(WoojinError::new("The type that can't be compared", crate::error::WoojinErrorKind::CannotCompare)),
    }
  }

  fn biggerthen(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self.value(), other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Bool(a > b)),
      (WoojinValue::Long(a), WoojinValue::Long(b)) => Ok(WoojinValue::Bool(a > b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Bool(a > b)),
      (WoojinValue::Double(a), WoojinValue::Double(b)) => Ok(WoojinValue::Bool(a > b)),
      _ => Err(WoojinError::new("The type that can't be compared", crate::error::WoojinErrorKind::CannotCompare))
    }
  }

  fn smallerthen(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self.value(), other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Bool(a < b)),
      (WoojinValue::Long(a), WoojinValue::Long(b)) => Ok(WoojinValue::Bool(a < b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Bool(a < b)),
      (WoojinValue::Double(a), WoojinValue::Double(b)) => Ok(WoojinValue::Bool(a < b)),
      _ => Err(WoojinError::new("The type that can't be compared", crate::error::WoojinErrorKind::CannotCompare))
    }
  }

  fn biggerthen_equal(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self.value(), other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Bool(a >= b)),
      (WoojinValue::Long(a), WoojinValue::Long(b)) => Ok(WoojinValue::Bool(a >= b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Bool(a >= b)),
      (WoojinValue::Double(a), WoojinValue::Double(b)) => Ok(WoojinValue::Bool(a >= b)),
      _ => Err(WoojinError::new("The type that can't be compared", crate::error::WoojinErrorKind::CannotCompare))
    }
  }

  fn smallerthen_equal(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self.value(), other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Bool(a <= b)),
      (WoojinValue::Long(a), WoojinValue::Long(b)) => Ok(WoojinValue::Bool(a <= b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Bool(a <= b)),
      (WoojinValue::Double(a), WoojinValue::Double(b)) => Ok(WoojinValue::Bool(a <= b)),
      _ => Err(WoojinError::new("The type that can't be compared", crate::error::WoojinErrorKind::CannotCompare))
    }
  }

  fn add(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self.value(), other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Int(a + b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float(a + b)),
      (WoojinValue::Float(a), WoojinValue::Int(b)) => Ok(WoojinValue::Float(a + (b as f32))),
      (WoojinValue::Int(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float((a as f32) + b)),
      (WoojinValue::String(a), WoojinValue::String(b)) => Ok(WoojinValue::String(format!("{}{}", a, b))),
      _ => Err(WoojinError::new("The type that can't be added", crate::error::WoojinErrorKind::CannotAdd)),
    }  
  }

  fn sub(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self.value(), other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Int(a - b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float(a - b)),
      (WoojinValue::Float(a), WoojinValue::Int(b)) => Ok(WoojinValue::Float(a - (b as f32))),
      (WoojinValue::Int(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float((a as f32) - b)),
      _ => Err(WoojinError::new("The type that can't be subtracted", crate::error::WoojinErrorKind::CannotSubtract)),
    }  
  }

  fn mul(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    match (self.value(), other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(WoojinValue::Int(a * b)),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float(a * b)),
      (WoojinValue::Float(a), WoojinValue::Int(b)) => Ok(WoojinValue::Float(a * (b as f32))),
      (WoojinValue::Int(a), WoojinValue::Float(b)) => Ok(WoojinValue::Float((a as f32) * b)),
      (WoojinValue::String(left), WoojinValue::Int(right)) => Ok(WoojinValue::String(left.as_str().repeat(right as usize))),
      (WoojinValue::String(left), WoojinValue::Long(right)) => Ok(WoojinValue::String(left.as_str().repeat(right as usize))),
      _ => Err(WoojinError::new("The type that can't be multiplied!", crate::error::WoojinErrorKind::CannotMultiply)),
    }
  }

  fn div(&self, other: &WoojinValue) -> WoojinResult<WoojinValue> {
    #[allow(non_snake_case)]
    let DivisionZeroError: WoojinError = WoojinError::new("It cannot be divided by 0.0", crate::error::WoojinErrorKind::DivisionByZero);
    match (self.value(), other.value()) {
      (WoojinValue::Int(a), WoojinValue::Int(b)) => Ok(if b != 0 { WoojinValue::Int(a / b) } else { return Err(DivisionZeroError) }),
      (WoojinValue::Float(a), WoojinValue::Float(b)) => Ok(if b != 0.0 { WoojinValue::Float(a / b) } else {return Err(DivisionZeroError)}),
      (WoojinValue::Float(a), WoojinValue::Int(b)) => Ok(if b != 0 { WoojinValue::Float(a / (b as f32)) } else {return Err(DivisionZeroError)}),
      (WoojinValue::Int(a), WoojinValue::Float(b)) => Ok(if b != 0.0 { WoojinValue::Float((a as f32) / b) } else {return Err(DivisionZeroError)}),
      _ => Err(WoojinError::new("an indivisible type!", crate::error::WoojinErrorKind::CannotDivide)),
    }
  }
}