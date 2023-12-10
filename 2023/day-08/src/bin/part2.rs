use std::collections::HashMap;

fn main() {
  let input = include_str!("./input1.txt");
  let output = part2(input);
  dbg!(output);
}

fn walk(start_node: &str, nodes: &HashMap<&str, (&str, &str)>, instructions: &str) -> u64 {
  let mut iter = instructions.chars().cycle();
  let mut steps: u64 = 0;
  let mut current = nodes.get(start_node).expect("Tuple");
  while let Some(c) = iter.next() {
    steps += 1;
    let picked: &str;
    // println!("{:?} {c} {steps}", current);
    if c == 'L' {
      picked = current.0
    } else {
      picked = current.1
    }
    if picked.ends_with("Z") {
      break;
    }
    // print!("{picked}");
    current = nodes.get(picked).expect("Tuple");
  }

  println!("{steps}");
  return steps;
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
  while b != 0 {
    let t = b;
    b = a % b;
    a = t;
  }
  a
}

fn lcm(a: u64, b: u64) -> u64 {
  a / gcd(a, b) * b
}

fn part2(input: &str) -> u64 {
  let lines = input.lines().collect::<Vec<&str>>();
  let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

  lines.iter().skip(2).for_each(|line| {
    nodes.insert(&line[0..3], (&line[7..10], &line[12..15]));
  });

  nodes
    .keys()
    .filter(|k| k.ends_with("A"))
    .map(|start_node| walk(start_node, &nodes, &lines[0]))
    .fold(1 as u64, |acc, num| lcm(acc, num))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 6);
  }
}
