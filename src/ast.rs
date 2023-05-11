use crate::{types::WoojinValue, variable::VariableOption, parser::Calc};

#[derive(Debug, Clone)]
pub(crate) enum Statements {
  Comment(String),
  Calc(Calc),
  Print { values: Vec<Box<Statements>> },
  Println { values: Vec<Box<Statements>> },
  Assignment { name: String, value: Box<Statements> },
  Input { question: Box<Statements> },
  Let {
    name: String,
    stmt: Box<Statements>,
    option: VariableOption,
  },
  Roar { value: WoojinValue },
  Yee { code: i32 },
  Value { value: WoojinValue },
  Sleep { value: Box<Statements> }
}