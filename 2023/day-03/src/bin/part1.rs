fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1(input: &str) -> u32 {
  let nesw: [(i32, i32); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
  ];
  let mut score = 0;

  let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let height = board.len() as i32;
  let width = board.iter().map(|row| row.len()).max().unwrap_or(0) as i32;
  let mut is_part = false;
  let mut num = String::new();

  for (row, li) in board.iter().enumerate() {
    if is_part {
      println!("{num}");
      score += num.parse::<u32>().expect("Positive number");
      is_part = false;
    }
    num.clear();
    for (col, ch) in li.iter().enumerate() {
      if ch.is_ascii_digit() {
        for (add_r, add_c) in nesw {
          if is_part {
            continue;
          }

          let r = row as i32 + add_r;
          let c = col as i32 + add_c;
          if (0 <= r && r < height) && (0 <= c && c < width) {
            let other = board[r as usize][c as usize];
            is_part = is_part || !other.is_ascii_digit() && other != '.';
          }
        }
        num.push(*ch);
      } else if !num.is_empty() {
        if is_part {
          println!("{num}");
          score += num.parse::<u32>().expect("Positive number");
          is_part = false;
        }
        num.clear();
      }
    }
  }
  return score;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 4361);
  }
}
