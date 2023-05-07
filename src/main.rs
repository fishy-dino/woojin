use std::{process, io::{Read}};
use woojin::{error, exec, Statements};
use nom::{IResult};

pub type NomResult<'a, T> = IResult<&'a str, T>;

fn main() {
  let args: Vec<String> = std::env::args().collect::<Vec<String>>();
  if args.len() < 2 {
    eprintln!("Usage: woojin [file]");
    process::exit(1);
  }
  let path: &String = &args[1];
  let code: String = match read_file(path) {
    Ok(code) => code,
    Err(err) => { error(&err); }
  };
  run(code);
}

fn read_file(path: &str) -> std::io::Result<String> {
  let mut file = std::fs::File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
}

fn run(code: impl ToString) {
  let lines = code.to_string()
    .replace(";", "\n")
    .split("\n")
    .map(|a| a.trim().to_string())
    .filter(|a| a.len() > 0)
    .collect::<Vec<String>>();
  let mut program: Program = Program::new();
  for line in lines {
    let res: Result<Statements, error::WoojinError> = woojin::token::tokenizer(&line);
    match res {
      Ok(v) => {program.statements.push(v)},
      Err(err) => { error(&err); }
    }
  }
  for stmt in program.statements.iter() {
    if let Err(err) = exec(stmt) { error(&err); }
  }
}
struct Program { statements: Vec<Statements> }

impl Program {
  fn new() -> Program { Program { statements: vec![] } }
}