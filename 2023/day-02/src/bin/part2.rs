use std::collections::HashMap;

fn main() {
  let input = include_str!("./input1.txt");
  let output = part2(input);
  dbg!(output);
}

// 12 red cubes, 13 green cubes, and 14 blue cubes
fn part2(input: &str) -> i32 {
  let score: i32 = input
    .lines()
    .map(|line| line.split(' ').collect::<Vec<&str>>())
    .map(|parts| {
      let mut colors = HashMap::from([('r', 0), ('g', 0), ('b', 0)]);

      parts[2..].iter().enumerate().for_each(|(index, part)| {
        if let Ok(num) = part.parse::<i32>() {
          if let Some(color) = parts[index + 3].chars().nth(0) {
            colors
              .entry(color)
              .and_modify(|entry| *entry = i32::max(*entry, num))
              .or_insert(num);
          }
        }
      });
      return colors.values().product::<i32>();
    })
    .sum();

  return score;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 8);
  }
}
