use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("./input1.txt");
  let output = part2(input);
  dbg!(output);
}

fn count_matching(line: &str) -> usize {
  let parts = line.split([':', '|']).collect::<Vec<&str>>();
  let selected: HashSet<&str> = parts[1].trim().split_whitespace().collect();
  let card_nums: HashSet<&str> = parts[2].trim().split_whitespace().collect();

  selected.intersection(&card_nums).count()
}

fn part2(input: &str) -> usize {
  let mut counts = HashMap::<usize, usize>::new();

  let score = input.lines().enumerate().fold(0, |acc, (index, line)| {
    let repeat = *counts.entry(index + 1).or_insert(1);
    let matches = count_matching(line);

    for i in index + 2..index + 2 + matches {
      println!("c{} m{matches} n{i} r{repeat}", index + 1);

      counts
        .entry(i)
        .and_modify(|entry| {
          println!("{entry}");
          *entry += repeat
        })
        .or_insert(repeat + 1);
    }

    acc + repeat
  });

  score
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
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 30);
  }
}
