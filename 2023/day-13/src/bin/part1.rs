fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

fn detect_rows(board: &Vec<Vec<char>>) -> usize {
  let len = board.len();

  for i in 1..len {
    let mut all = true;
    if i > len / 2 {
      for k in 0..len - i {
        all &= board[i - k - 1] == board[i + k];
        if !all {
          break;
        }
      }
    } else {
      for k in 0..i {
        all &= board[k] == board[2 * i - k - 1];
        if !all {
          break;
        }
      }
    }
    if !all {
      continue;
    }
    return i;
  }
  0
}

fn detect_cols(board: &Vec<Vec<char>>) -> usize {
  let len = board[0].len();
  for i in 1..len {
    let mut all = true;
    if i > len / 2 {
      for k in 0..len - i {
        all &= board.iter().all(|row| row[i - k - 1] == row[i + k]);
        if !all {
          break;
        }
      }
    } else {
      for k in 0..i {
        all &= board.iter().all(|row| row[k] == row[2 * i - k - 1]);
        if !all {
          break;
        }
      }
    }
    if !all {
      continue;
    }
    return i;
  }
  0
}

fn print_board(board: &Vec<Vec<char>>) {
  board.iter().for_each(|line| {
    line.iter().for_each(|&c| {
      print!("{c}");
    });
    println!();
  });
  println!();
}

fn part1(input: &str) -> usize {
  let mut ans = 0;
  input.split_terminator("\n\n").for_each(|block| {
    let board: Vec<Vec<char>> = block.lines().map(|line| line.chars().collect()).collect();

    let rows_score = detect_rows(&board);
    let col_score = detect_cols(&board);
    ans += rows_score * 100;
    ans += col_score;

    println!("row: {rows_score}");
    println!("col: {col_score}");

    print_board(&board);
  });

  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "..#..#.##
..#..#.##
#...#..#.
##.#..##.
#.##.#.#.
###.#...#
##..#####
.....#..#
.#...#..#
##..#####
###.#...#";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 405);
  }
}
