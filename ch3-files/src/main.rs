//! Simulating files one step at a time.

#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
  Open,
  Closed,
}

/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
  state: FileState,
}

impl Display for FileState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      FileState::Open => write!(f, "OPEN"),
      FileState::Closed => write!(f, "CLOSED"),
    }
  }
}

impl Display for File {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "<{} ({})>", self.name, self.state)
  }
}

impl File {
  /// New files are assumed to be empty, but a name is required.
  fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
      state: FileState::Closed,
    }
  }
  /// Returns the file's length in bytes.
  pub fn len(&self) -> usize {
    self.data.len()
  }

  /// Returns the file's name.
  pub fn name(&self) -> String {
    self.name.clone()
  }
}

fn main() {
  let f6 = File::new("f6.txt");
  println!("{:?}", f6);
  println!("{}", f6);
}
