use std::collections::HashSet;

fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1(input: &str) -> u32 {
  let score = input.lines().fold(0, |acc, line| {
    let parts = line.split([':', '|']).collect::<Vec<&str>>();
    let selected: HashSet<&str> = parts[1].trim().split_whitespace().collect();
    let card_nums: HashSet<&str> = parts[2].trim().split_whitespace().collect();

    let count = selected.intersection(&card_nums).count() as u32;
    println!("{count}");
    if count == 0 {
      return acc;
    }
    return acc + 2_u32.pow(count - 1);
  });

  return score;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 13);
  }
}
