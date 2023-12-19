fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part2(input);

  dbg!(output1);
}

#[derive(Debug)]
struct Segment {
  y0: i64,
  x0: i64,
  y1: i64,
  x1: i64,
  delta: i64,
}

fn parse_input(input: &str) -> Vec<(usize, i64)> {
  input
    .lines()
    .filter_map(|line| {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() == 3 {
        let distance = i64::from_str_radix(&parts[2][2..=6], 16).unwrap();
        let index = usize::from_str_radix(&parts[2][7..=7], 16).unwrap();

        println!("{:?} - {index} {distance}", parts[2]);
        Some((index, distance))
      } else {
        None
      }
    })
    .collect()
}

fn part2(input: &str) -> i64 {
  let dir: Vec<(i64, i64)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

  let parts = parse_input(input);

  let mut point = (0, 0);
  let mut segments: Vec<Segment> = vec![];

  for (dir_i, n) in parts.iter() {
    let dd = dir[*dir_i];
    segments.push(Segment {
      y0: point.0,
      x0: point.1,
      y1: point.0 + dd.0 * n,
      x1: point.1 + dd.1 * n,
      delta: *n,
    });
    point = (point.0 + dd.0 * n, point.1 + dd.1 * n);
  }

  // println!("{:?}", segments);
  let area = segments
    .iter()
    .map(|s| s.x0 * s.y1 - s.y0 * s.x1 + s.delta)
    .sum::<i64>();

  // The mystery of the +1. Is this the last point which overlaps in horizontal and vertical when going back to 0?
  (area as f64 / 2.0 + 1.0) as i64
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
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 952408144115);
  }
}
