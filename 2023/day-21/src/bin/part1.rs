use std::collections::HashSet;

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

fn part1(input: &str) -> usize {
  let nesw: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
  let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut now: HashSet<(usize, usize)> = HashSet::from([(board.len() / 2, board.len() / 2)]);
  let mut then: HashSet<(usize, usize)> = HashSet::new();

  for _ in 0..26501365 {
    // println!("{:?}", now);
    for (row, col) in now {
      for (dr, dc) in nesw {
        let (new_r, new_c) = ((row as i32 + dr) as usize, (col as i32 + dc) as usize);
        let char_option: Option<&char> = board.get(new_r).and_then(|line| line.get(new_c));
        if char_option.is_none() {
          continue;
        }
        let val = char_option.unwrap();
        if val == &'.' || val == &'S' {
          then.insert((new_r, new_c));
        }
      }
    }
    now = then.clone();
    then.clear();
  }
  now.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 16);
  }
}
