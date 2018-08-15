use common::*;

pub struct Node {
  _library: Library,
}

impl Node {
  pub fn new(library: Library) -> Node {
    Node { _library: library }
  }

  pub fn run(self) -> Result<(), Error> {
    Ok(())
  }
}
