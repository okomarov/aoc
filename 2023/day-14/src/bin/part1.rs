fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
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

fn tilt_north(board: &mut Vec<Vec<char>>) {
  for r in 0..board.len() {
    for c in 0..board[0].len() {
      if board[r][c] == 'O' {
        let mut new_r = r.clone();
        while new_r > 0 && board[new_r - 1][c] == '.' {
          new_r -= 1;
        }
        if new_r != r {
          board[new_r][c] = 'O';
          board[r][c] = '.';
        }
      }
    }
  }
}

fn part1(input: &str) -> usize {
  let mut board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let row_len = board.len();
  print_board(&board);

  tilt_north(&mut board);
  print_board(&board);

  let mut ans = 0;
  for r in 0..row_len {
    for c in 0..board[0].len() {
      if board[r][c] == 'O' {
        ans += row_len - r;
      }
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 136);
  }
}
