use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

fn parse_input(input: &str) -> Vec<(char, i32, &str)> {
  input
    .lines()
    .filter_map(|line| {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() == 3 {
        let direction = parts[0].chars().next()?;
        let distance = parts[1].parse::<i32>().ok()?;
        let color = parts[2];
        Some((direction, distance, color))
      } else {
        None
      }
    })
    .collect()
}

fn part1(input: &str) -> usize {
  let dir: HashMap<char, (i32, i32)> =
    HashMap::from([('U', (-1, 0)), ('R', (0, 1)), ('D', (1, 0)), ('L', (0, -1))]);
  let mut tile = (0, 0);
  let mut path: HashSet<(i32, i32)> = HashSet::from([tile]);

  let parts = parse_input(input);
  for (d, n, _) in parts.iter() {
    let dd = dir.get(d).unwrap();
    for _ in 0..*n {
      tile = (tile.0 + dd.0, tile.1 + dd.1);
      path.insert(tile);
    }
  }

  let mut queue = vec![(-18, -4)];

  let ((min_row, max_row), (min_col, max_col)) = find_min_max(&path);
  println!("({min_row}, {max_row}), ({min_col}, {max_col})");
  // print_path(&path);

  while let Some(tile) = queue.pop() {
    if path.contains(&tile)
      || tile.0 < min_row
      || tile.0 > max_row
      || tile.1 < min_col
      || tile.1 > max_col
    {
      continue;
    }
    path.insert(tile);
    for (_, d) in dir.iter() {
      queue.push((tile.0 + d.0, tile.1 + d.1));
    }
  }

  print_path(&path);
  path.len()
}

fn find_min_max(path: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
  let mut min_row = i32::MAX;
  let mut max_row = i32::MIN;
  let mut min_col = i32::MAX;
  let mut max_col = i32::MIN;

  for &(row, col) in path {
    if row < min_row {
      min_row = row;
    }
    if row > max_row {
      max_row = row;
    }
    if col < min_col {
      min_col = col;
    }
    if col > max_col {
      max_col = col;
    }
  }

  ((min_row, max_row), (min_col, max_col))
}

fn print_path(path: &HashSet<(i32, i32)>) {
  let ((min_row, max_row), (min_col, max_col)) = find_min_max(path);

  for r in min_row..=max_row {
    for c in min_col..=max_col {
      if path.contains(&(r, c)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!();
  }
  println!();
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
