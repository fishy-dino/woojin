use std::{fs::File, io::{BufReader, BufRead}};
use woojin::{
  error::{WoojinError, WoojinErrorKind},
};

const INDENT: usize = 2;

fn main() {
  let args: Vec<String> = std::env::args().collect::<Vec<String>>();
  if args.len() < 2 {
    WoojinError::new(
      "Give Me File!\nUsage: woojin [file]",
      WoojinErrorKind::FileNotFound,
    )
    .exit();
  }
  let path: &String = &args[1];
  if !path.ends_with(".wj")&&!path.ends_with(".woojin") {
    WoojinError::new(
      "I don't think it's woojin file(.wj, .woojin)",
      WoojinErrorKind::UnsupportedExtension,
    )
    .exit();
  }
  let file: File = match File::open(path) {
    Ok(file) => file,
    Err(_) => WoojinError::new(
      "File Not Found",
      WoojinErrorKind::FileNotFound
    ).exit(),
  };
  let reader: BufReader<File> = BufReader::new(file);
  let mut lines: Vec<(usize, String)> = Vec::new();
  for line in reader.lines() {
    if let Ok(line) = line {
      if line.is_empty() { continue; }
      let line: Vec<&str> = line.split(" ".repeat(INDENT).as_str()).collect::<Vec<&str>>();
      lines.push((line.len()-1, line[line.len()-1].to_string()))
    }
  }
  if lines[lines.len()-1].0 != 0||!lines[lines.len()-1].1.starts_with("yee") {
    lines.push((0, "yee 0".to_string()));
  }
  woojin::run(lines);
}

