use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("./input1.txt");
  let output = part2(input);
  dbg!(output);
}

fn part2(input: &str) -> u32 {
  let directions: [(i32, i32); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
  ];

  let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut gear_map: HashMap<String, HashSet<u32>> = HashMap::new();
  let mut num = String::new();
  let mut gears: HashSet<String> = Default::default();

  for (row, li) in board.iter().enumerate() {
    if !num.is_empty() {
      update_gear_map(&mut num, &mut gears, &mut gear_map);
    }
    for (col, ch) in li.iter().enumerate() {
      if ch.is_ascii_digit() {
        num.push(*ch);
        find_connected_gears(&directions, &board, row, col, &mut gears);
      } else if !num.is_empty() {
        update_gear_map(&mut num, &mut gears, &mut gear_map);
      }
    }
  }

  return gear_map
    .values()
    .filter_map(|set| {
      if set.len() > 1 {
        Some(set.iter().product::<u32>())
      } else {
        None
      }
    })
    .sum();
}

fn find_connected_gears(
  directions: &[(i32, i32)],
  board: &[Vec<char>],
  row: usize,
  col: usize,
  gears: &mut HashSet<String>,
) {
  for (add_r, add_c) in directions.iter() {
    let r = row as i32 + add_r;
    let c = col as i32 + add_c;
    if let Some(this_row) = board.get(r as usize) {
      if let Some(&'*') = this_row.get(c as usize) {
        let key = format!("{r}{c}");
        gears.insert(key);
      }
    }
  }
}

fn update_gear_map(
  num: &mut String,
  gears: &mut HashSet<String>,
  gear_map: &mut HashMap<String, HashSet<u32>>,
) {
  let _num = num.parse::<u32>().expect("Number");
  for gear in gears.iter() {
    gear_map
      .entry(gear.clone())
      .and_modify(|set| {
        set.insert(_num);
      })
      .or_insert_with(|| HashSet::from([_num]));
  }
  num.clear();
  gears.clear();
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
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 467835);
  }
}
