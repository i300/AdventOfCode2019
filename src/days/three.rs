use crate::days::Day;
use crate::Result;
use std::cmp::max;
use std::cmp::min;

pub struct Three {
  filename: &'static str
}

impl Three {
  pub fn new(filename: &'static str) -> Three {
    Three { filename }
  }
}

type Point = (i64,i64);

fn manhattan(a: Point) -> i64 {
  a.0.abs() + a.1.abs()
} 

// magic code from http://flassari.is/2008/11/line-line-intersection-in-cplusplus/
fn intersection(a: (Point,Point), b: (Point,Point)) -> Option<Point> {
  let x1 = (a.0).0;
  let x2 = (a.1).0;
  let x3 = (b.0).0;
  let x4 = (b.1).0;
  let y1 = (a.0).1;
  let y2 = (a.1).1;
  let y3 = (b.0).1;
  let y4 = (b.1).1;

  let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
  if d == 0 {
    None
  } else {
    let pre = x1 * y2 - y1 * x2;
    let post = x3 * y4 - y3 * x4;
    let x = (pre * (x3 - x4) - (x1 - x2) * post) / d;
    let y = (pre * (y3 - y4) - (y1 - y2) * post) / d;

    if x < min(x1, x2) || x > max(x1, x2) || x < min(x3, x4) || x > max(x3, x4) ||
       y < min(y1, y2) || y > max(y1, y2) || y < min(y3, y4) || y > max(y3, y4) {
      None
    } else {
      Some((x, y))
    }
  }
}

fn line_points(input: &str) -> Result<Vec<(Point, i64)>> {
  let moves: Vec<String> = input.split(",").map(|s| s.to_string()).collect();
  let mut points: Vec<(Point, i64)> = Vec::new();
  let mut current_coord = (0, 0);
  let mut current_steps = 0;
  for op in moves {
    let dir = match op.chars().next() {
      Some('U') => (0, 1),
      Some('D') => (0, -1),
      Some('R') => (1, 0),
      Some('L') => (-1, 0),
      Some(_) => (0, 0),
      None => (0, 0)
    };

    let value = (&op[1..]).parse::<i64>()?;
    current_steps += value;
    current_coord = (current_coord.0 + dir.0 * value, current_coord.1 + dir.1 * value);
    points.push((current_coord, current_steps));
  };

  Ok(points)
}

fn dist(a: Point, b: Point) -> i64 {
  (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f64).sqrt().floor() as i64
}

impl Day for Three {
  fn run(&self) -> Result<String> {
    let mut points_vec: Vec<Vec<(Point, i64)>> = Vec::new();

    let contents = crate::util::read_file(self.filename)?;
    let lines = contents.lines();
    for line in lines {
      let points = line_points(line)?;
      points_vec.push(points);
    }

    let mut intersections: Vec<(Point, i64)> = Vec::new();
    let first_points = &points_vec[0];
    let second_points = &points_vec[1];
    for i in 1..first_points.len() {
      for j in 1..second_points.len() {
        let a0_steps = first_points[i-1].1;
        let a0 = first_points[i-1].0;
        let a1 = first_points[i].0;
        
        let b0_steps = second_points[j-1].1;
        let b0 = second_points[j-1].0;
        let b1 = second_points[j].0;
        if let Some(x) = intersection((a0, a1), (b0, b1)) {
          let steps = a0_steps + b0_steps + dist(a0, x) + dist(b0, x);
          intersections.push((x, steps));
        }
      }
    }

    // Part 1
    let mut smallest_dist = i64::max_value();
    for x in intersections.iter() {
      let dist = manhattan(x.0);
      if dist < smallest_dist {
        smallest_dist = dist;
      }
    }

    // Part 2
    let mut smallest_steps = i64::max_value();
    for x in intersections.iter() {
      let steps = x.1;
      if steps < smallest_steps {
        smallest_steps = steps;
      }
    }
    Ok(format!("{}", smallest_steps))
  }
}