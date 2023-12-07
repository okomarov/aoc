fn main() {
  let input = include_str!("./input1.txt");
  let output = part2(input);
  dbg!(output);
}
#[derive(Clone, Debug)]
struct Mapping {
  source: u64,
  destination: u64,
  increment: u64,
}

fn part2(input: &str) -> u64 {
  let parts = input.split("\n\n").collect::<Vec<&str>>();
  let mut mappings = parts[1..].iter().map(make_map).collect::<Vec<_>>();

  let seeds = parts[0]
    .split_whitespace()
    .skip(1)
    .map(|s| s.parse::<u64>().expect("Number"))
    .collect::<Vec<u64>>();

  let mut score = u64::MAX;

  for m in mappings.iter_mut() {
    m.sort_by_key(|k| k.destination);
  }

  let mut location = mappings.last().cloned().expect("Location");

  location.insert(
    0,
    Mapping {
      source: 0,
      destination: 0,
      increment: location[0].destination,
    },
  );

  println!("{:?}", location);

  'outer: for loc in location {
    println!("{:?}", loc);
    for s in loc.source..loc.source + loc.increment {
      score = s;
      let mut i = s;
      for m in mappings.iter().rev().skip(1) {
        for row in m {
          if row.destination <= i && i < (row.destination + row.increment) {
            i = row.source + i - row.destination;
            break;
          }
        }
      }
      for pair in seeds.chunks(2) {
        if pair[0] <= i && i < pair[0] + pair[1] {
          break 'outer;
        }
      }
    }
  }

  return score;
}

fn make_map(part: &&str) -> Vec<Mapping> {
  part
    .lines()
    .skip(1)
    .map(|line| {
      let nums = line
        .split_whitespace()
        .map(|n| n.parse::<u64>().expect("Number"))
        .collect::<Vec<u64>>();

      Mapping {
        destination: nums[0],
        source: nums[1],
        increment: nums[2],
      }
    })
    .collect::<Vec<Mapping>>()
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
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 46);
  }
}
