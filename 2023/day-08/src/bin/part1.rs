use std::collections::HashMap;

fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1(input: &str) -> usize {
  let lines = input.lines().collect::<Vec<&str>>();
  let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

  lines.iter().skip(2).for_each(|line| {
    nodes.insert(&line[0..3], (&line[7..10], &line[12..15]));
  });

  let mut iter = lines[0].chars().cycle();

  let mut steps = 0;
  let mut current = nodes.get("AAA").expect("Tuple");
  while let Some(c) = iter.next() {
    steps += 1;
    let picked: &str;
    // println!("{:?} {c} {steps}", current);
    if c == 'L' {
      picked = current.0
    } else {
      picked = current.1
    }
    if picked == "ZZZ" {
      break;
    }
    // print!("{picked}");
    current = nodes.get(picked).expect("Tuple");
  }

  return steps;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 6);
  }
}
