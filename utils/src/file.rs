use std::fs::{read_to_string};
use std::path::PathBuf;

pub fn read_file(file_path: &str) -> String {
  let mut srcdir = PathBuf::from("./files/");
  srcdir.push(file_path);
  srcdir.set_extension("txt");

  read_to_string(srcdir).expect("Unable to read file")
}