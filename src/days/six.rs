use crate::days::Day;
use crate::Result;
use crate::StringError;
use std::collections::HashMap;

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
    
    // build graph
    for line in lines {
      let orbit_pair: Vec<&str> = line.split(")").collect();
      
      let parent = match orbit_pair.get(0) {
          Some(v) => *v,
          None => return Err(Box::new(StringError::new("Orbit pair does not contain orbited.".to_string())))
      }.to_string();

      let child = match orbit_pair.get(1) {
          Some(v) => *v,
          None => return Err(Box::new(StringError::new("Orbit pair does not contain orbiter.".to_string())))
      }.to_string();

      if let Some(node) = graph.get_mut(&child) {
        node.parent = parent.clone();
      } else {
        let node = Node { parent: parent.clone(), children: Vec::new() };
        graph.insert(child.clone(), node);
      } 

      if let Some(node) = graph.get_mut(&parent) {
        node.children.push(child.clone());
      } else {
        let mut node = Node { parent: String::new(), children: Vec::new() };
        node.children.push(child.clone());
        graph.insert(parent.clone(), node);
      }
    }

    // Part 1
    let mut sum_distances = 0;
    for (key, _) in &graph {
      sum_distances += find_com(&graph, key)?;
    }

    // Part 2
    let you = "YOU".to_string();
    let san_distance = find_san(&graph, &you, &you)? - 3; // Minus 1 for SAN, 1 for YOU, and 1 for the node SAN orbits
    Ok(format!("Sum distances = {}, # of transfers to SAN = {}", sum_distances.to_string(), san_distance.to_string()))
  }
}

fn find_com(graph: &HashMap<String, Node>, start: &String) -> Result<i32> {
  // If we are COM, then the distance is 0
  if start == "COM" {
    return Ok(0)
  }
  // Otherwise recurse down the tree
  if let Some(node) = graph.get(start) {
    Ok(1 + find_com(graph, &node.parent)?)
  } else {
    return Err(Box::new(StringError::new(format!("Node {} not found in graph.", start))))
  }
}

fn find_san(graph: &HashMap<String, Node>, start: &String, from: &String) -> Result<i32> {
  if start == "SAN" {
    return Ok(1)
  }
  if let Some(node) = graph.get(start) {
    let mut search_set: Vec<String> = node.children.to_vec();
    search_set.push(node.parent.clone());

    for node in search_set.iter() {
      if node == from || node.is_empty() {
        continue
      } 
      let result = find_san(&graph, node, start)?;
      if result >= 1 {
        return Ok(result + 1)
      }
    }

    Ok(0)
  } else {
    return Err(Box::new(StringError::new(format!("Node {} not found in graph.", start))))
  }
}

//----

struct Node {
  parent: String,
  children: Vec<String>
}