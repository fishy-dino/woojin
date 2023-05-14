use crate::{types::{WoojinValue, WoojinValueKind}, variable::VariableOption, calc::Calc};

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
    kind: WoojinValueKind,
    stmt: Box<Statements>,
    option: VariableOption,
  },
  If {
    condition: Box<Statements>,
    stmt: Vec<Box<Statements>>,
    else_stmt: Vec<Box<Statements>>,
  },
  Roar { value: WoojinValue },
  Yee { code: i32 },
  Value { value: WoojinValue },
  Sleep { value: Box<Statements> }
}