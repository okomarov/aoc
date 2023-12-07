use std::collections::HashMap;

fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}

fn count_chars(s: &str) -> HashMap<char, u64> {
  let mut counts = HashMap::new();
  for ch in s.chars() {
    *counts.entry(ch).or_insert(0) += 1;
  }
  counts
}

fn compare_strings(a: &str, b: &str, order: &str) -> std::cmp::Ordering {
  for (char_a, char_b) in a.chars().zip(b.chars()) {
    let index_a = order.find(char_a).unwrap_or(usize::MAX);
    let index_b = order.find(char_b).unwrap_or(usize::MAX);

    if index_a != index_b {
      return index_a.cmp(&index_b);
    }
  }
  println!("Hello");

  a.len().cmp(&b.len())
}

fn part1(input: &str) -> u64 {
  let order = "AKQJT98765432".chars().rev().collect::<String>();
  let mut hands: HashMap<&str, u64> = HashMap::new();
  let mut groups: Vec<Vec<&str>> = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
  input.lines().for_each(|line| {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    hands.insert(parts[0], parts[1].parse::<u64>().expect("number"));
    let m = count_chars(parts[0]);
    let n = m.keys().len();
    // Highest
    if n == 5 {
      groups[0].push(parts[0]);
    // One pair
    } else if n == 4 {
      groups[1].push(parts[0]);
    } else if n == 3 {
      // Double pair
      if m.values().any(|v| v == &2) {
        groups[2].push(parts[0]);
        // Three of a kind
      } else {
        groups[3].push(parts[0]);
      }
    } else if n == 2 {
      // Four of a kind
      if m.values().any(|v| v == &4) {
        groups[5].push(parts[0]);
        // Full house
      } else if m.values().any(|v| v == &2) {
        groups[4].push(parts[0]);
        // Three of a kind
      } else {
        groups[3].push(parts[0]);
      }
      // Five of a kind
    } else {
      groups[6].push(parts[0]);
    }
  });

  for group in &mut groups {
    group.sort_by(|a, b| compare_strings(a, b, &order));
  }
  println!("{:?}", groups);

  groups
    .into_iter()
    .flatten()
    .enumerate()
    .map(|(index, k)| {
      let bid = hands.get(k).expect("Bid");
      let index = index as u64 + 1;
      println!("{k} {index} {bid}");
      bid * index
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 6592);
  }
}
