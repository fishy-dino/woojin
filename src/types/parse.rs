use nom::{
  branch::alt,
  bytes::complete::{
    tag, escaped, is_not
  },
  character::complete::{char, digit1},
  combinator::{map, map_res, opt, recognize},
  sequence::{delimited, pair, tuple},
  IResult,
};

use crate::{
  parser::{parse_variable_name}
};

use super::WoojinValue;

// Integer(signed)
pub(crate) fn parse_int(input: &str) -> IResult<&str, i32> {
  map_res(
    recognize(pair(
      opt(alt((char('+'), char('-')))),
      digit1,
    )),
    |s: &str| s.parse::<i32>(),
  )(input)
}

pub(crate) fn parse_float(input: &str) -> IResult<&str, f32> {
  map_res(
    recognize(tuple((
      opt(alt((char('+'), char('-')))),
      pair(digit1, char('.')),
      digit1,
    ))),
    |s: &str| s.parse::<f32>(),
  )(input)
}

pub(crate) fn parse_string(input: &str) -> IResult<&str, String> {
  let re: regex::Regex = regex::Regex::new(r"\\").unwrap();
  let result: (&str, &str) = delimited(
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
pub(crate) fn parse_bool(input: &str) -> IResult<&str, bool> {
  alt((
    map(tag("uglyguri"), |_| true),
    map(tag("beautifulguri"), |_| false),
  ))(input)
}

// parse value
pub(crate) fn parse_value(input: &str) -> IResult<&str, WoojinValue> {
  alt((
    map(parse_string, WoojinValue::String),
    map(parse_float, WoojinValue::Float),
    map(parse_int, WoojinValue::Int),
    map(parse_bool, WoojinValue::Bool),
    map(parse_variable_name, WoojinValue::Var),
  ))(input)
}