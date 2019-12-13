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

impl Day for Three {
  fn run(&self) -> Result<String> {
    let contents = crate::util::read_file(self.filename)?;
    let lines = contents.lines();
    let mut occupied_coords: Vec<(i32,i32)> = Vec::new();
    let mut intersections: Vec<(i32,i32)> = Vec::new();

    for line in lines {
      let mut current_coord = (0, 0);
      let moves: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
      for op in moves {
        let dir = match op.chars().next() {
          Some('U') => (0, 1),
          Some('L') => (-1, 0),
          Some('D') => (0, -1),
          Some('R') => (1, 0),
          Some(_) => (0, 0),
          None => (0, 0)
        };
        let value = (&op[1..2]).parse::<i32>()?;
        
        for _ in 0..value {
          current_coord = (current_coord.0 + dir.0, current_coord.1 + dir.1);
          if occupied_coords.contains(&current_coord) {
            intersections.push(current_coord);
          }
          occupied_coords.push(current_coord);
        }
      }
    }

    let mut smallest_intersection = (0, 0);
    for intersection in intersections {
      if intersection.0 + intersection.1 < smallest_intersection.0 + smallest_intersection.1 {
        smallest_intersection = intersection;
      }
    }

    Ok(format!("{}, {}", smallest_intersection.0, smallest_intersection.1))
  }
}