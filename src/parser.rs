use std::str::FromStr;
use regex::Regex;

use crate::{
  ast::Statements,
  NomResult, types::{WoojinValue, parse::parse_value, WoojinValueKind}, error::WoojinError, variable::VariableOption, exec, calc::{parse_calc, Calc}
};

use nom::{
  IResult,
  branch::{alt},
  bytes::complete::{
    tag,
    take_while_m_n,
    take_while1, take_while,
  },
  character::complete::{char, multispace1, multispace0},
  sequence::{preceded, terminated},
  combinator::{map, map_res, opt, value}
};

pub(crate) type WoojinResult<T> = Result<T, crate::error::WoojinError>;

pub(crate) fn parse_int(input: &str) -> Result<i32, std::num::ParseIntError> {
  i32::from_str_radix(input, 10)
}

pub(crate) fn yee(input: &str) -> NomResult<Statements> {
  let (input, _): (&str, &str) = tag("yee ")(input)?;
  let (input, sign): (&str, Option<&str>) = opt(tag("-"))(input)?;
  let (input, num): (&str, i32) = map_res(take_while_m_n(1, 10, |c: char| c.is_digit(10)), parse_int)(input)?;
  let num: i32 = if let Some(_) = sign { -num } else { num };
  Ok((input, Statements::Yee { code: num }))
}

pub(crate) fn vec2stmt(values: &Vec<&str>) -> WoojinResult<Vec<Box<Statements>>> {
  let mut result: Vec<Box<Statements>> = vec![];
  for value in values {
    let val: Statements = tokenize_line(value)?;
    result.push(Box::new(val));
  };
  Ok(result)
}

pub(crate) fn split_comma(input: &str) -> WoojinResult<Vec<&str>> {
  let values: Vec<&str> = if input.trim().contains(",") {
    let mut in_quotes: bool = false;
    let mut start: usize = 0;
    let mut result: Vec<&str> = vec![];
    let chars: std::iter::Enumerate<std::str::Chars> = input.trim().chars().enumerate();
    for (i, c) in chars {
      match c {
        '"' => in_quotes = !in_quotes,
        ',' if !in_quotes => {
          let value: &&str = &input[start..i].trim();
          result.push(value.to_owned()); // push owned value
          start = i + 1;
        }
        _ => {}
      }
    }
    let value: &&str = &input[start..].trim();
    result.push(value.to_owned()); // push owned value
    result
  } else {
    vec![input.trim()]
  };
  Ok(values)
}

pub(crate) fn print(input: &str) -> WoojinResult<Statements> {
  let input: &str = match tag::<&str, _, nom::error::Error<&str>>("print ")(input) {
    Ok((input, _)) => input.trim(),
    Err(_) => return Err(WoojinError::new("Invalid usage of print", crate::error::WoojinErrorKind::Unknown))
  };
  let values: Vec<&str> = split_comma(input)?;
  Ok(Statements::Print { values: vec2stmt(&values)? })
}

pub(crate) fn println(input: &str) -> WoojinResult<Statements> {
  let input: &str = match tag::<&str, _, nom::error::Error<&str>>("println ")(input) {
    Ok((input, _)) => input.trim(),
    Err(_) => return Err(WoojinError::new("Invalid usage of println", crate::error::WoojinErrorKind::Unknown))
  };
  let values: Vec<&str> = split_comma(input)?;
  Ok(Statements::Println{ values: vec2stmt(&values)? })
}

pub(crate) fn roar(input: &str) -> WoojinResult<Statements> {
  let input: &str = match tag::<&str, _, nom::error::Error<&str>>("roar ")(input) {
    Ok((input, _)) => input,
    Err(_) => return Err(WoojinError::new("Invalid usage of roar", crate::error::WoojinErrorKind::Unknown))
  };
  Ok(Statements::Roar { value: test(input)? })
}

pub(crate) fn input(i: &str) -> WoojinResult<Statements> {
  let input: &str = match tag::<&str, _, nom::error::Error<&str>>("input ")(i) {
    Ok((input, _)) => input,
    Err(_) => return Err(WoojinError::new("Invalid usage of input", crate::error::WoojinErrorKind::Unknown))
  };
  Ok(Statements::Input { question: Box::new(tokenize_line(&input.to_string())?) })
}

pub(crate) fn sleep(i: &str) -> WoojinResult<Statements> {
  let input: &str = match tag::<&str, _, nom::error::Error<&str>>("sleep ")(i) {
    Ok((input, _)) => input,
    Err(_) => return Err(WoojinError::new("Invalid usage of sleep", crate::error::WoojinErrorKind::Unknown))
  };
  Ok(Statements::Sleep { value: Box::new(tokenize_line(&input.to_string())?) })
}

pub(crate) fn parse_variable(input: &str) -> IResult<&str, (String, String, &str, bool)> {
  let (input, _): (&str, &str) = multispace0(input)?;
  let (input, mutable): (&str, bool) = alt((
      value(true, preceded(tag("let"), preceded(multispace1, tag("mut")))),
      value(false, preceded(tag("let"), multispace1)),
  ))(input)?;
  let (input, _): (&str, &str) = multispace0(input)?;
  let (input, var_name): (&str, String) = map(
      take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'),
      |s: &str| s.to_string(),
  )(input)?;
  let (input, _): (&str, &str) = multispace0(input)?;
  let (input, _): (&str, Option<&str>) = opt(tag(":"))(input)?;
  let (input, _): (&str, &str) = multispace0(input)?;
  let (input, var_type): (&str, Option<String>) = map(
      opt(take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_')),
      |s: Option<&str>| s.map(|s| s.to_string()),
  )(input)?;
  let var_type: String = if let Some(s) = var_type { s } else { "".to_string()};
  let (input, _): (&str, &str) = multispace0(input)?;
  let (input, _): (&str, char) = char('=')(input)?;
  let (input, _): (&str, &str) = multispace0(input)?;
  let (input, value): (&str, &str) = take_while1(|c: char| c != ';' && c != '\n')(input)?;
  Ok((input, (var_name, var_type, value, mutable)))
}

// pub(crate) fn parse_assignment(input: &str) -> WoojinResult<(String, String)>{
//   let splited: Vec<String> = input.split("=").map(|a| a.to_string()).collect();
//   if splited.len() < 2 { return Err(WoojinError::new("Invalid assignment", crate::error::WoojinErrorKind::InvaildAssignment)); }
//   let (_, varname) = parse_variable_name(splited[0].trim())?;
//   let value: String = splited[1].trim().to_string();
//   Ok((varname, value))
// }

fn parse_whitespace(input: &str) -> IResult<&str, &str> {
  take_while(|c: char| c.is_whitespace())(input)
}

fn parse_if_condition(input: &str) -> IResult<&str, &str> {
  preceded(tag("if "), terminated(take_while(|c: char| c != ':'), tag(":")))(input)
}

fn check_is_else(input: &str) -> IResult<&str, &str> {
  preceded(tag("else"), terminated(parse_whitespace, tag(":")))(input)
}

fn is_else(input: &str) -> bool {
  match check_is_else(input) {
    Ok(_) => true,
    Err(_) => false
  }
}

pub(crate) fn test(input: &str) -> WoojinResult<WoojinValue> {
  match tokenize_line(&input.to_string())? {
    Statements::Value { value: val } => Ok(val),
    a => match exec(&a) {
      Ok(val) => Ok(val),
      Err(e) => Err(e)
    }
  }
}

pub(crate) fn parse_variable_name(input: &str) -> IResult<&str, String> {
  let (input, a): (&str, &str) = preceded(char('$'), take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'))(input)?;
  Ok((input, a.to_string()))
}

pub(crate) fn tokenizer(lines: &Vec<(usize, String)>) -> WoojinResult<Vec<Statements>> {
  let mut pointer: usize = 0;
  let mut result: Vec<Statements> = vec![];
  while pointer < lines.len() {
    let (indent, line): &(usize, String) = &lines[pointer];
    let mut tokenized: Statements = tokenize_line(line)?.clone();
    match &mut tokenized {
      Statements::If { condition: _, stmt, else_stmt} => {
        let (if_stmt, e_stmt): (Vec<Box<Statements>>, Vec<Box<Statements>>) = parse_if(lines, &mut pointer, *indent)?;
        *stmt = if_stmt;
        *else_stmt = e_stmt;
        result.push(tokenized)
      }
      _ => result.push(tokenized),
    }
    pointer += 1;
  }
  Ok(result)
}

pub(crate) fn parse_if(lines: &Vec<(usize, String)>, pointer: &mut usize, indent: usize) -> WoojinResult<(Vec<Box<Statements>>, Vec<Box<Statements>>)> {
  let mut result: (Vec<Box<Statements>>, Vec<Box<Statements>>) = (vec![], vec![]);
  *pointer += 1;
  while *pointer < lines.len() {
    let (line_indent, line): &&(usize, String) = &lines.get(*pointer).ok_or(WoojinError::new("Invalid indent", crate::error::WoojinErrorKind::InvalidIndent))?;
    if *line_indent <= indent {
      if !is_else(line) {
        *pointer -= 1;
        return Ok(result);
      }
      result.1 = parse_else(lines, &mut *pointer, *line_indent)?;
      return Ok(result);
    }
    let mut tokenized: Statements = tokenize_line(line)?;
    match &mut tokenized {
      Statements::If { condition: _, stmt, else_stmt } => {
        let (if_stmt, e_stmt): (Vec<Box<Statements>>, Vec<Box<Statements>>) = parse_if(lines, &mut *pointer, *line_indent)?;
        *stmt = if_stmt;
        *else_stmt = e_stmt;
        result.0.push(Box::new(tokenized));
      }
      _ => result.0.push(Box::new(tokenized))
    }
    *pointer += 1;
  }
  Err(WoojinError::new("Parsing If statement failed", crate::error::WoojinErrorKind::IfParsingFailed))
}

pub(crate) fn parse_else(lines: &Vec<(usize, String)>, pointer: &mut usize, indent: usize) -> WoojinResult<Vec<Box<Statements>>> {
  let mut result: Vec<Box<Statements>> = vec![];
  *pointer += 1;
  while *pointer < lines.len() {
    let (line_indent, line): &&(usize, String) = &lines.get(*pointer).ok_or(WoojinError::new("Invalid indent", crate::error::WoojinErrorKind::InvalidIndent))?;
    if *line_indent <= indent {
      *pointer = *pointer-1;
      return Ok(result);
    }
    let mut tokenized: Statements = tokenize_line(line)?;
    match &mut tokenized {
      Statements::If { condition: _, stmt, else_stmt } => {
        let (if_stmt, e_stmt): (Vec<Box<Statements>>, Vec<Box<Statements>>) = parse_if(lines, &mut *pointer, *line_indent)?;
        *stmt = if_stmt;
        *else_stmt = e_stmt;
        result.push(Box::new(tokenized));
      }
      _ => result.push(Box::new(tokenized))
    }
    *pointer += 1;
  }
  Err(WoojinError::new("Parsing Else statement failed", crate::error::WoojinErrorKind::ElseParsingFailed))
}

pub(crate) fn tokenize_line(line: &str) -> WoojinResult<Statements> {
  let line: String = line.to_string().trim().to_string();
  let chvar_reg = Regex::new(r"(?m)\$[a-zA-Z_]{1}[a-zA-Z0-9_]*\s*=\s*").unwrap();
  match line {
    line if line == "" => Ok(Statements::Value { value: WoojinValue::String("".to_string()) }),
    line if line.starts_with("if") => {
      let (_, condition): (&str, &str) = parse_if_condition(&line)?;
      let condition: Statements = tokenize_line(&condition.to_string())?;
      Ok(Statements::If { condition: Box::new(condition), stmt: Vec::new(), else_stmt: Vec::new() })
    },
    line if chvar_reg.is_match(line.as_str()) => {
      let splited = line.split("=").map(|a| a.to_string()).collect::<Vec<String>>();
      let (_, varname) = parse_variable_name(splited[0].trim())?;
      let stmts: Statements = tokenize_line(splited[1..].join("=").trim())?;
      Ok(Statements::Assignment { name: varname, value: Box::new(stmts) })
    },
    line if line.starts_with("else") => {Ok(Statements::Value { value: WoojinValue::Unit })},
    line if line.starts_with("//") => Ok(Statements::Comment(line[2..].trim().to_string())),
    line if line.starts_with("yee") => match yee(&line)? { (_, a) => { return Ok(a); }, }
    line if line.starts_with("println") => Ok(println(&line)?),
    line if line.starts_with("print") => Ok(print(&line)?),
    line if line.starts_with("roar") => Ok(roar(&line)?),
    line if line.starts_with("input") => Ok(input(&line)?),
    line if line.starts_with("sleep") => Ok(sleep(&line)?),
    line if line.starts_with("let") => {
      let (_, (var_name, kind, input, mutable)): (&str, (String, String, &str, bool)) = parse_variable(&line)?;
      let stmts: Statements = tokenize_line(&input.to_string())?;
      Ok(Statements::Let {
        name: var_name,
        stmt: Box::new(stmts),
        kind: if kind.is_empty() { WoojinValueKind::Any } else { WoojinValueKind::from_str(&kind)? },
        option: VariableOption::new(Some(mutable), None)
      })
    },
    _ => match parse_calc(line.as_str()) {
      Ok(val) => {
        match val.1 {
          Calc::Value(a) => Ok(Statements::Value {value: a}),
          _ => Ok(Statements::Calc(val.1))
        }
      },
      _ => match parse_value(line.as_str()) {
        Ok(val) => Ok(Statements::Value {value: val.1}),
        Err(_) => Err(WoojinError::new(format!("Unknown token \"{}\"", line), crate::error::WoojinErrorKind::UnknownToken))
      }
    }
  }
}