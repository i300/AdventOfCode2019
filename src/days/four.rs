use crate::days::Day;
use crate::Result;

pub struct Four {
  filename: &'static str
}

impl Four {
  pub fn new(filename: &'static str) -> Four {
    Four { filename }
  }
}

fn has_adjacent(n: u32) -> Option<()> {
    let str_value = n.to_string();
    let mut adj_chars: Vec<u32> = Vec::new();
    let mut chars = str_value.chars();
    let mut last_ch = chars.next()?;
    let mut adj_count = 1;
    for ch in chars {
        if ch == last_ch {
            adj_count += 1;
        } else {
            if adj_count > 1 {
                adj_chars.push(adj_count);
                adj_count = 1;
            }
        }
        last_ch = ch;
    }
    if adj_count > 1 {   
        adj_chars.push(adj_count);
    }
    for adj in adj_chars {
        if adj == 2 {
            return Some(());
        }
    }
    None
}

fn doesnt_decrease(n: u32) -> Option<()> {
    let str_value = n.to_string();
    let mut chars = str_value.chars();
    let mut last_ch = chars.next()?;
    for ch in chars {
        if ch < last_ch {
            return None;
        }
        last_ch = ch;
    };
    Some(())
}

impl Day for Four {
    fn run(&self) -> Result<String> {
        let contents = crate::util::read_file(self.filename)?;
        let range: Vec<u32> = match contents.split("-").map(|s| s.parse::<u32>()).collect() {
            Ok(s) => s,
            Err(e) => return Err(Box::new(e))
        };

        let low = range[0];
        let high = range[1];

        let mut possible_values: Vec<u32> = Vec::new();
        for n in low..high {
            if let Some(_) = doesnt_decrease(n) {
                if let Some(_) = has_adjacent(n) {
                    possible_values.push(n);
                }
            }
        }

        Ok(format!("{}", possible_values.len()))
    }
}