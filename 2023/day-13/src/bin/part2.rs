fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part2(input);

  dbg!(output1);
}

fn detect_rows(board: &Vec<Vec<char>>, old: &usize) -> usize {
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
    if !all || &i == old {
      continue;
    }
    return i;
  }
  0
}

fn detect_cols(board: &Vec<Vec<char>>, old: &usize) -> usize {
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
    if !all || &i == old {
      continue;
    }
    return i;
  }
  0
}

fn print_board(board: &Vec<Vec<char>>, pos: Option<&(usize, usize)>) {
  board.iter().enumerate().for_each(|(row, line)| {
    line.iter().enumerate().for_each(|(col, &c)| {
      if pos.map_or(false, |x| x == &(row, col)) {
        print!("@");
      } else {
        print!("{c}");
      }
    });
    println!();
  });
  println!();
}

fn part2(input: &str) -> usize {
  let mut ans = 0;
  input.split_terminator("\n\n").for_each(|block| {
    let board: Vec<Vec<char>> = block.lines().map(|line| line.chars().collect()).collect();
    let old_row_score = detect_rows(&board, &99);
    let old_col_score = detect_cols(&board, &99);
    println!("old row: {old_row_score}");
    println!("old col: {old_col_score}");
    print_board(&board, None);

    let row_len = board.len();
    let col_len = board[0].len();

    'outer: for r in 0..row_len {
      for c in 0..col_len {
        let mut new_board = board.clone();
        new_board[r][c] = if new_board[r][c] == '.' { '#' } else { '.' };
        let row_score = detect_rows(&new_board, &old_row_score);
        let col_score = detect_cols(&new_board, &old_col_score);

        // print_board(&new_board, Some(&(r, c)));

        if old_row_score != row_score && row_score > 0 {
          println!("{r},{c}");
          println!("row: {row_score}");
          ans += row_score * 100;
          print_board(&new_board, Some(&(r, c)));
          break 'outer;
        } else if old_col_score != col_score && col_score > 0 {
          println!("{r},{c}");
          println!("col: {col_score}");

          ans += col_score;
          print_board(&new_board, Some(&(r, c)));
          break 'outer;
        }
      }
    }
  });

  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "....##...
...#..#..
##.#..#.#
###....##
..##..##.
###....##
####..###
..##..##.
####..#.#
..######.
..#.##.#.";
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 400);
  }
}
