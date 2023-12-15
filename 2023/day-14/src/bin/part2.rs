use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

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

fn tilt_south(board: &mut Vec<Vec<char>>) {
  for r in (0..board.len()).rev() {
    for c in 0..board[0].len() {
      if board[r][c] == 'O' {
        let mut new_r = r.clone();
        while new_r < board.len() - 1 && board[new_r + 1][c] == '.' {
          new_r += 1;
        }
        if new_r != r {
          board[new_r][c] = 'O';
          board[r][c] = '.';
        }
      }
    }
  }
}

fn tilt_west(board: &mut Vec<Vec<char>>) {
  for r in 0..board.len() {
    for c in 0..board[0].len() {
      if board[r][c] == 'O' {
        let mut new_c = c.clone();
        while new_c > 0 && board[r][new_c - 1] == '.' {
          new_c -= 1;
        }
        if new_c != c {
          board[r][new_c] = 'O';
          board[r][c] = '.';
        }
      }
    }
  }
}

fn tilt_east(board: &mut Vec<Vec<char>>) {
  for r in 0..board.len() {
    for c in (0..board[0].len()).rev() {
      if board[r][c] == 'O' {
        let mut new_c = c.clone();
        while new_c < board[0].len() - 1 && board[r][new_c + 1] == '.' {
          new_c += 1;
        }
        if new_c != c {
          board[r][new_c] = 'O';
          board[r][c] = '.';
        }
      }
    }
  }
}

fn get_o(board: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let mut pos = vec![];
  for r in 0..board.len() {
    for c in 0..board[0].len() {
      if board[r][c] == 'O' {
        pos.push((r, c))
      }
    }
  }
  pos
}
fn part1(input: &str) -> usize {
  let mut boards: HashMap<u64, usize> = HashMap::new();
  let mut board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let row_len = board.len();
  print_board(&board);

  for i in 0..81 + (1000000000 - 81) % 51 {
    let pos = get_o(&board);
    let pos_hash = hash_vec(&pos);

    // Find the phase = i-first_seen, then the nth configuration is: first_seen + [(n-first_seen) % phase]
    // Adjust the loop length afterwards
    if let Some(first_seen) = boards.get(&pos_hash) {
      println!("{first_seen} {i}");
      break;
    } else {
      boards.insert(pos_hash, i);
    }
    println!("{i} {pos_hash}");

    tilt_north(&mut board);
    tilt_west(&mut board);
    tilt_south(&mut board);
    tilt_east(&mut board);
  }
  // print_board(&board);

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

fn hash_vec(vec: &Vec<(usize, usize)>) -> u64 {
  let mut hasher = DefaultHasher::new();
  vec.hash(&mut hasher);
  hasher.finish()
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
