use std::{fs::File, io::Read};
use woojin::{
  error::{WoojinError, WoojinErrorKind},
};

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
  if !path.ends_with(".wj") {
    WoojinError::new(
      "I don't think it's woojin file(.wj)",
      WoojinErrorKind::UnsupportedExtension,
    )
    .exit();
  }
  let mut file: File = match File::open(path) {
    Ok(file) => file,
    Err(_) => WoojinError::new("File Not Found", WoojinErrorKind::FileNotFound).exit(),
  };
  let mut contents: String = String::new();
  if let Err(error) = file.read_to_string(&mut contents) {
    WoojinError::new(
      format!("File Read Error: {}", error),
      WoojinErrorKind::FailReadFailure,
    )
    .exit();
  }
  let lines: Vec<String> = contents
    .replace(";", "\n")
    .split("\n")
    .map(|line| line.trim().to_string())
    .filter(|line| !line.is_empty())
    .collect();
  woojin::run(lines);
}

