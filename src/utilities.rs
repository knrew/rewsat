use std::{
  error::Error,
  fs,
  io::{prelude::*, BufReader},
  path::Path,
};

pub fn read_file<P: AsRef<Path>>(file: P) -> Result<Vec<String>, Box<dyn Error>> {
  let f = fs::File::open(file)?;
  let reader = BufReader::new(f);

  let mut result = vec![];

  for line in reader.lines() {
    result.push(line?.trim().to_string());
  }

  Ok(result)
}
