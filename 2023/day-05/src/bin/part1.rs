fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1(input: &str) -> u64 {
  let parts = input.split("\n\n").collect::<Vec<&str>>();
  let mappings = parts[1..].iter().map(make_map).collect::<Vec<_>>();

  let seeds = parts[0]
    .split(" ")
    .skip(1)
    .map(|s| s.parse::<u64>().expect("Number"));

  let mut score = u64::MAX;

  for s in seeds {
    let mut i = s;
    for m in &mappings {
      for row in m {
        if row[1] <= i && i < (row[1] + row[2]) {
          i = row[0] + i - row[1];
          break;
        }
      }
    }
    score = score.min(i);
  }

  return score;
}

fn make_map(part: &&str) -> Vec<[u64; 3]> {
  part
    .lines()
    .skip(1)
    .map(|line| {
      let nums = line
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("Number"))
        .collect::<Vec<u64>>();

      if nums.len() != 3 {
        panic!("Expected exactly 3 numbers per line");
      }

      [nums[0], nums[1], nums[2]]
    })
    .collect::<Vec<[u64; 3]>>()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 35);
  }
}
