use std::collections::HashMap;

fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}

// 12 red cubes, 13 green cubes, and 14 blue cubes
fn part1(input: &str) -> usize {
  let color_max: HashMap<char, u32> = HashMap::from([('r', 12), ('g', 13), ('b', 14)]);

  let score = input
    .lines()
    .map(|line| line.split(' ').collect::<Vec<&str>>())
    .enumerate()
    .filter(|(_, parts)| {
      parts[2..]
        .iter()
        .enumerate()
        .all(|(index, part)| match part.parse::<u32>() {
          Ok(num) => {
            if let Some(color) = parts[index + 3].chars().nth(0) {
              return color_max.get(&color).is_some_and(|max| return num <= *max);
            } else {
              return true;
            }
          }
          Err(_) => true,
        })
    })
    .fold(0, |acc, (index, _)| acc + index + 1);

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
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 8);
  }
}
