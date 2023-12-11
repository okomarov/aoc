use std::collections::HashSet;

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input, &2);
  let output2 = part1(input, &1000000);
  dbg!(output1);
  dbg!(output2);
}

fn collect_galaxies(board: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let mut positions = Vec::new();

  for (row, line) in board.iter().enumerate() {
    for (col, &c) in line.iter().enumerate() {
      if c == '#' {
        positions.push((row, col));
      }
    }
  }

  positions
}

fn collect_empty_rows(board: &Vec<Vec<char>>) -> HashSet<usize> {
  board
    .iter()
    .enumerate()
    .filter_map(|(idx, row)| {
      if row.iter().all(|&c| c == '.') {
        Some(idx)
      } else {
        None
      }
    })
    .collect()
}

fn collect_empty_cols(board: &Vec<Vec<char>>) -> HashSet<usize> {
  let num_columns = board[0].len();
  let mut empty_columns = vec![true; num_columns];
  board.iter().for_each(|row| {
    row
      .iter()
      .enumerate()
      .for_each(|(col, c)| empty_columns[col] = empty_columns[col] && c == &'.')
  });

  (0..num_columns).filter(|c| empty_columns[*c]).collect()
}

fn part1(input: &str, multiplier: &usize) -> usize {
  let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let galaxies = collect_galaxies(&board);
  let empty_rows = collect_empty_rows(&board);
  let empty_cols = collect_empty_cols(&board);

  let mut num = 0;
  for (index, galaxy) in galaxies.iter().enumerate() {
    for other in &galaxies[index..galaxies.len()] {
      let row_num = empty_rows
        .iter()
        .filter(|x| {
          if galaxy.0 > other.0 {
            &&other.0 < x && x < &&galaxy.0
          } else {
            &&galaxy.0 < x && x < &&other.0
          }
        })
        .count();
      let col_num = empty_cols
        .iter()
        .filter(|x| {
          if galaxy.1 > other.1 {
            &&other.1 < x && x < &&galaxy.1
          } else {
            &&galaxy.1 < x && x < &&other.1
          }
        })
        .count();
      let count = (galaxy.0).abs_diff(other.0)
        + (galaxy.1).abs_diff(other.1)
        + row_num * (multiplier - 1)
        + col_num * (multiplier - 1);
      num += count;
    }
  }
  num
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let score = part1(input, &2);
    print!("{}", score);
    assert_eq!(score, 374);
  }
}
