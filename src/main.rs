use std::{io::{Read}};
use woojin::{error, exec, Statements};

fn main() {
  let args: Vec<String> = std::env::args().collect::<Vec<String>>();
  if args.len() < 2 { error(&"Usage: woojin [file]"); }
  let path: &String = &args[1];
  let code: String = match read_file(path) {
    Ok(code) => code,
    Err(err) => { error(&err); }
  };
  run(code);
}

fn read_file(path: &str) -> std::io::Result<String> {
  if !path.ends_with(".wj") { error(&"Error: file extension must be .wj"); }
  let mut file: std::fs::File = std::fs::File::open(path)?;
  let mut contents: String = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
}

fn run(code: impl ToString) {
  let lines: Vec<String> = code.to_string()
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
    if let Err(err) = exec(stmt) { err.exit(); }
  }
}
struct Program { statements: Vec<Statements> }

impl Program {
  fn new() -> Program { Program { statements: vec![] } }
}