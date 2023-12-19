use std::collections::HashMap;

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

#[derive(Debug)]
struct Segment {
  y0: i64,
  x0: i64,
  y1: i64,
  x1: i64,
}

fn parse_input(input: &str) -> Vec<(char, i64, &str)> {
  input
    .lines()
    .filter_map(|line| {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() == 3 {
        let direction = parts[0].chars().next()?;
        let distance = parts[1].parse::<i64>().ok()?;
        let color = parts[2];
        println!("{distance}");
        Some((direction, distance, color))
      } else {
        None
      }
    })
    .collect()
}

fn part1(input: &str) -> usize {
  let dir: HashMap<char, (i64, i64)> =
    HashMap::from([('U', (-1, 0)), ('R', (0, 1)), ('D', (1, 0)), ('L', (0, -1))]);
  let parts = parse_input(input);
  let mut point = (0, 0);
  let mut segments: Vec<Segment> = vec![];

  for (dir_i, n, _) in parts.iter() {
    let dd = dir[dir_i];
    segments.push(Segment {
      y0: point.0,
      x0: point.1,
      y1: point.0 + dd.0 * n,
      x1: point.1 + dd.1 * n,
    });
    point = (point.0 + dd.0 * n, point.1 + dd.1 * n);
  }

  println!("{:?}", segments);
  let area = segments
    .iter()
    .map(|s| s.x0 * s.y1 - s.y0 * s.x1 + (s.x1 - s.x0).abs() + (s.y1 - s.y0).abs())
    .sum::<i64>();

  (area as f64 / 2.0) as usize
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 62);
  }
}
