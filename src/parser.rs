use crate::{
  ast::Statements,
  NomResult, types::{WoojinValue, parse::parse_value}, error::WoojinError, variable::VariableOption, exec
};

use nom::{
  IResult,
  branch::{alt},
  multi::{many0, fold_many0},
  bytes::complete::{
    tag,
    take_while_m_n,
    take_while1,
  },
  character::complete::{
    char, multispace1, multispace0, space0
  },
  sequence::{pair, preceded, delimited},
  combinator::{map, map_res, opt, value}
};

pub(crate) type WoojinResult<T> = Result<T, crate::error::WoojinError>;

pub(crate) fn parse_int(input: &str) -> Result<i32, std::num::ParseIntError> {
  i32::from_str_radix(input, 10)
}

pub(crate) fn yee(input: &str) -> NomResult<Statements> {
  let (input, _) = tag("yee ")(input)?;
  let (input, sign) = opt(tag("-"))(input)?;
  let (input, num) = map_res(take_while_m_n(1, 10, |c: char| c.is_digit(10)), parse_int)(input)?;
  let num = if let Some(_) = sign { -num } else { num };
  Ok((input, Statements::Yee { code: num }))
}

pub(crate) fn vec2stmt(values: &Vec<&str>) -> WoojinResult<Vec<Box<Statements>>> {
  let mut result: Vec<Box<Statements>> = vec![];
  for value in values {
    let val = tokenizer(value)?;
    result.push(Box::new(val));
  };
  Ok(result)
}

pub(crate) fn split_comma(input: &str) -> WoojinResult<Vec<&str>> {
  let values: Vec<&str> = if input.trim().contains(",") {
    let mut in_quotes = false;
    let mut start = 0;
    let mut result = vec![];
    let chars = input.trim().chars().enumerate();
    for (i, c) in chars {
      match c {
        '"' => in_quotes = !in_quotes,
        ',' if !in_quotes => {
          let value = &input[start..i].trim();
          result.push(value.to_owned()); // push owned value
          start = i + 1;
        }
        _ => {}
      }
    }
    // 마지막 값 추가
    let value = &input[start..].trim();
    result.push(value.to_owned()); // push owned value
    result
  } else {
    vec![input.trim()]
  };
  Ok(values)
}

pub(crate) fn print(input: &str) -> WoojinResult<Statements> {
  let input = match tag::<&str, _, nom::error::Error<&str>>("print ")(input) {
    Ok((input, _)) => input.trim(),
    Err(_) => return Err(WoojinError::new("Invalid usage of print", crate::error::WoojinErrorKind::Unknown))
  };
  let values: Vec<&str> = split_comma(input)?;
  Ok(Statements::Print { values: vec2stmt(&values)? })
}

pub(crate) fn println(input: &str) -> WoojinResult<Statements> {
  let input = match tag::<&str, _, nom::error::Error<&str>>("println ")(input) {
    Ok((input, _)) => input.trim(),
    Err(_) => return Err(WoojinError::new("Invalid usage of println", crate::error::WoojinErrorKind::Unknown))
  };
  let values: Vec<&str> = split_comma(input)?;
  Ok(Statements::Println{ values: vec2stmt(&values)? })
}

pub(crate) fn roar(input: &str) -> WoojinResult<Statements> {
  let input = match tag::<&str, _, nom::error::Error<&str>>("roar ")(input) {
    Ok((input, _)) => input,
    Err(_) => return Err(WoojinError::new("Invalid usage of roar", crate::error::WoojinErrorKind::Unknown))
  };
  Ok(Statements::Roar { value: test(input)? })
}

pub(crate) fn input(i: &str) -> WoojinResult<Statements> {
  let input = match tag::<&str, _, nom::error::Error<&str>>("input ")(i) {
    Ok((input, _)) => input,
    Err(_) => return Err(WoojinError::new("Invalid usage of input", crate::error::WoojinErrorKind::Unknown))
  };
  Ok(Statements::Input { question: Box::new(tokenizer(&input.to_string())?) })
}

pub(crate) fn sleep(i: &str) -> WoojinResult<Statements> {
  let input = match tag::<&str, _, nom::error::Error<&str>>("sleep ")(i) {
    Ok((input, _)) => input,
    Err(_) => return Err(WoojinError::new("Invalid usage of sleep", crate::error::WoojinErrorKind::Unknown))
  };
  Ok(Statements::Sleep { value: Box::new(tokenizer(&input.to_string())?) })
}

pub(crate) fn parse_variable(input: &str) -> IResult<&str, (String, &str, bool)> {
  let (input, _) = multispace0(input)?;
  let (input, mutable) = alt((
    value(true, preceded(tag("let"), preceded(multispace1, tag("mut")))),
    value(false, preceded(tag("let"), multispace1)),
  ))(input)?;
  let (input, _) = multispace0(input)?;
  let (input, var_name) = map(
    pair(
      take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'),
      many0(preceded(multispace1, take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'))),
    ),
    |(first, rest)| {
      let mut name = String::from(first);
      for s in rest {
        name.push(' ');
        name.push_str(s);
      }
      name
    },
  )(input)?;
  let (input, _) = multispace0(input)?;
  let (input, _) = char('=')(input)?;
  let (input, _) = multispace0(input)?;
  Ok((input, (var_name.to_string(), input, mutable)))
}

pub(crate) fn parse_assignment(input: &str) -> IResult<&str, (String, &str, String)> {
  let (input, _) = multispace0(input)?;
  let (input, var_name) = map(
    pair(
      preceded(char('$'), take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_')),
      many0(preceded(multispace1, take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'))),
    ),
    |(first, rest)| {
      let mut name = String::from(first);
      for s in rest {
        name.push(' ');
        name.push_str(s);
      }
      name
    },
  )(input)?;
  let (input, _) = multispace0(input)?;
  let (input, _) = char('=')(input)?;
  let (input, _) = multispace0(input)?;
  let (input, value) = map(
    pair(
      take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'),
      many0(preceded(multispace1, take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'))),
    ),
    |(first, rest)| {
      let mut val = String::from(first);
      for s in rest {
        val.push(' ');
        val.push_str(s);
      }
      val
    },
  )(input)?;
  let (input, _) = multispace0(input)?;
  Ok((input, (var_name.to_string(), input, value.to_string())))
}

pub(crate) fn test(input: &str) -> WoojinResult<WoojinValue> {
  match tokenizer(&input.to_string())? {
    Statements::Value { value: val } => Ok(val),
    a => match exec(&a) {
      Ok(val) => Ok(val),
      Err(e) => Err(e)
    }
  }
}

pub(crate) fn parse_variable_name(input: &str) -> IResult<&str, String> {
  let (input, a) = preceded(char('$'), take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_'))(input)?;
  Ok((input, a.to_string()))
}

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
    map(parse_value, Calc::Value),
    delimited(
      char('('),
      delimited(space0, parse_expr, space0),
      char(')'),
    ),
  ))(input)
}

pub(crate) fn parse_term(input: &str) -> IResult<&str, Calc> {
  let (input, init) = parse_primary(input)?;
  fold_many0(
    pair(alt((tag("*"), tag("/"))), parse_primary),
    move || init.clone(),
    |acc, (op, val)| match op {
        "*" => Calc::Mul(Box::new(acc), Box::new(val)),
        "/" => Calc::Div(Box::new(acc), Box::new(val)),
        _ => unreachable!(),
    },
  )(input)
}

pub(crate) fn parse_expr(input: &str) -> IResult<&str, Calc> {
  let (input, init) = parse_term(input)?;
  fold_many0(
    pair(alt((tag("+"), tag("-"))), parse_term),
    move || init.clone(),
    |acc, (op, val)| match op {
      "+" => Calc::Add(Box::new(acc), Box::new(val)),
      "-" => Calc::Sub(Box::new(acc), Box::new(val)),
      _ => unreachable!(),
    },
  )(input)
}


pub(crate) fn parse_calc(input: &str) -> IResult<&str, Calc> {
  delimited(space0, parse_expr, space0)(input)
}

pub(crate) fn tokenizer(line: &impl ToString) -> Result<Statements, crate::error::WoojinError> {
  let line = line.to_string().trim().to_string();
  match line {
    line if line == "" => Ok(Statements::Value { value: WoojinValue::String("".to_string()) }),
    line if line.starts_with("//") => Ok(Statements::Comment(line[2..].trim().to_string())),
    line if line.starts_with("yee") => match yee(&line)? { (_, a) => { return Ok(a); }, }
    line if line.starts_with("println") => Ok(println(&line)?),
    line if line.starts_with("print") => Ok(print(&line)?),
    line if line.starts_with("roar") => Ok(roar(&line)?),
    line if line.starts_with("input") => Ok(input(&line)?),
    line if line.starts_with("sleep") => Ok(sleep(&line)?),
    line if line.starts_with("let") => {
      let (_, (var_name, input, mutable)) = parse_variable(&line)?;
      let stmts = tokenizer(&input.to_string())?;
      Ok(Statements::Let {
        name: var_name,
        stmt: Box::new(stmts),
        option: VariableOption::new(Some(mutable), None)
      })
    },
    line if line.starts_with("$") && line.contains("=") => {
      let (_, (var_name, _, value)) = parse_assignment(&line)?;
      let stmts = tokenizer(&value.to_string())?;
      Ok(Statements::Assignment { name: var_name, value: Box::new(stmts) })
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