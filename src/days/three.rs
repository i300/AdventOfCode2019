use crate::days::Day;
use crate::Result;

pub struct Three {
  filename: &'static str
}

impl Three {
  pub fn new(filename: &'static str) -> Three {
    Three { filename }
  }
}

fn manhattan(a: (i32,i32)) -> i32 {
  a.0.abs() + a.1.abs()
}

impl Day for Three {
  fn run(&self) -> Result<String> {
    let contents = crate::util::read_file(self.filename)?;
    let lines = contents.lines();
    let mut occupied_coords: Vec<(i32,i32,bool)> = Vec::new();
    let mut intersections: Vec<(i32,i32)> = Vec::new();
    
    let mut first_line = false;
    for line in lines {
      first_line = !first_line;
      let mut current_coord = (0, 0, first_line);
      let moves: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
      for op in moves {
        let dir = match op.chars().next() {
          Some('U') => (0, 1),
          Some('D') => (0, -1),
          Some('R') => (1, 0),
          Some('L') => (-1, 0),
          Some(_) => (0, 0),
          None => (0, 0)
        };

        let value = (&op[1..]).parse::<i32>()?;
        for _ in 0..value {
          current_coord = (current_coord.0 + dir.0, current_coord.1 + dir.1, first_line);
          let other_coord = (current_coord.0, current_coord.1, !current_coord.2);
          if !first_line && occupied_coords.contains(&other_coord) {
            intersections.push((current_coord.0, current_coord.1));
            println!("{},{},{}", current_coord.0, current_coord.1, current_coord.2);
          }
          occupied_coords.push(current_coord);
        }
      }
    }

    let mut smallest_intersection: (i32,i32) = (99999, 99999);
    for intersection in intersections {
      if manhattan(intersection) < manhattan(smallest_intersection) {
        smallest_intersection = intersection;
      }
    }
    
    Ok(format!("{}, {}, {}", smallest_intersection.0, smallest_intersection.1, manhattan(smallest_intersection)))
  }
}