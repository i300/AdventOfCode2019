use crate::days::Day;
use crate::Result;
use crate::StringError;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Six {
    filename: &'static str
}

impl Six {
  pub fn new(filename: &'static str) -> Six {
    Six { filename }
  }
}

impl Day for Six {
  fn run(&self) -> Result<String> {
    let contents = crate::util::read_file(self.filename)?;
    let lines = contents.lines();

    let mut graph: HashMap<String, Node> = HashMap::new();
    
    // build tree
    for line in lines {
      let orbitPair: Vec<&str> = line.split(")").collect();
      let orbited = match orbitPair.get(0) {
          Some(v) => *v,
          None => return Err(Box::new(StringError::new("Orbit pair does not contain orbited.".to_string())))
      }.to_string();

      let orbiter = match orbitPair.get(1) {
          Some(v) => *v,
          None => return Err(Box::new(StringError::new("Orbit pair does not contain orbiter.".to_string())))
      }.to_string();

      if let Some(node) = graph.get_mut(&orbited) {
        node.edges.push(orbiter.clone());
      } else {
        let mut node = Node { value: orbited.clone(), edges: Vec::new() };
        node.edges.insert(0, orbiter.clone());
        graph.insert(orbited, node);
      }

      if !graph.contains_key(&orbiter) {
        let node = Node { value: orbiter.clone(), edges: Vec::new() };
        graph.insert(orbiter, node);
      }
    }

    Ok(String::from("123"))
  }
}

//----

struct Node {
  value: String,
  edges: Vec<String>
}