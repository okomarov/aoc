use std::{
  cmp::max,
  cmp::min,
  collections::{HashMap, VecDeque},
};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part2(input);

  dbg!(output1);
}
#[derive(Clone, Debug)]
struct Cond {
  raw: String,
  letter: char,
  is_less: Option<bool>,
  than: Option<usize>,
  next: Option<String>,
  is_terminal: bool,
}

fn parse_input(input: &str) -> HashMap<String, VecDeque<Cond>> {
  let sections: Vec<&str> = input.split("\n\n").collect();

  let mut flows: HashMap<String, VecDeque<Cond>> = sections[0]
    .lines()
    .filter_map(|line| {
      let items: Vec<&str> = line.split(['{', '}']).collect();
      if items.len() > 1 {
        Some((
          items[0].to_string(),
          items[1].split(",").map(parse_condition).collect(),
        ))
      } else {
        None
      }
    })
    .collect();

  flows.insert(
    'A'.to_string(),
    VecDeque::from([Cond {
      raw: 'A'.to_string(),
      letter: 'A',
      is_terminal: true,
      is_less: None,
      than: None,
      next: None,
    }]),
  );

  flows.insert(
    'R'.to_string(),
    VecDeque::from([Cond {
      raw: 'R'.to_string(),
      letter: 'R',
      is_terminal: true,
      is_less: None,
      than: None,
      next: None,
    }]),
  );

  flows
}

fn parse_condition(s: &str) -> Cond {
  let parts: Vec<&str> = s.split(':').collect();
  if parts.len() >= 2 {
    let val = parts[0].get(2..).unwrap().parse::<usize>().unwrap();
    let next = parts[1].to_string();
    let is_less = parts[0].chars().nth(1).unwrap() == '<';

    Cond {
      raw: s.to_string(),
      letter: parts[0].chars().nth(0).unwrap(),
      is_less: Some(is_less),
      than: Some(val),
      next: Some(next),
      is_terminal: false,
    }
  } else {
    Cond {
      raw: s.to_string(),
      letter: ' ',
      is_less: None,
      than: None,
      next: Some(s.to_string()),
      is_terminal: true,
    }
  }
}

fn ncombs(letters: &HashMap<char, (usize, usize)>) -> usize {
  letters
    .values()
    .fold(1, |acc, (lb, ub)| acc * (ub - lb + 1))
}

fn part2(input: &str) -> usize {
  let flows = parse_input(input);

  // println!("{:?}", parts);
  // println!("{:?}", flows);

  let mut letters: HashMap<char, (usize, usize)> = HashMap::from([
    ('x', (1, 4000)),
    ('m', (1, 4000)),
    ('a', (1, 4000)),
    ('s', (1, 4000)),
  ]);

  // ncombs(&letters)

  evaluate_flow(flows.get("in").unwrap(), &mut letters, &flows)
}

fn evaluate_flow(
  flow: &VecDeque<Cond>,
  letters: &mut HashMap<char, (usize, usize)>,
  flows: &HashMap<String, VecDeque<Cond>>,
) -> usize {
  flow
    .iter()
    .map(|cond| {
      println!("{:?}, {:?}", &cond.raw, &letters);

      if cond.is_terminal {
        match cond.letter {
          'A' => {
            println!("{:?}", ncombs(&letters));
            ncombs(&letters)
          }
          'R' => {
            println!("rejected");
            0
          }
          _ => {
            println!("Adding {:?}", cond.next);
            let next = cond.next.as_ref().unwrap();
            let next_flow = flows.get(next).unwrap();
            evaluate_flow(&next_flow, letters, flows)
          }
        }
      } else {
        println!("Updating bounds");
        evaluate_conditional(cond, letters, flows)
      }
    })
    .sum()
}

fn evaluate_conditional(
  cond: &Cond,
  letters: &mut HashMap<char, (usize, usize)>,
  flows: &HashMap<String, VecDeque<Cond>>,
) -> usize {
  let next_letters = &mut letters.clone();
  let (next_lb, next_ub) = next_letters.get_mut(&cond.letter).unwrap();
  let (lb, ub) = letters.get_mut(&cond.letter).unwrap();
  let val = cond.than.unwrap();
  let is_less = cond.is_less.unwrap();

  if is_less {
    *lb = max(val, *lb);
    if *next_ub > val {
      *next_ub = val - 1;

      let next = cond.next.as_ref().unwrap();
      println!("Going into {:?}", next);
      let next_flow = flows.get(next).unwrap();
      return evaluate_flow(&next_flow, next_letters, flows);
    }
  } else {
    *ub = min(val, *ub);
    if *next_lb < val {
      *next_lb = val + 1;
      let next = cond.next.as_ref().unwrap();
      println!("Going into {:?}", next);
      let next_flow = flows.get(next).unwrap();
      return evaluate_flow(&next_flow, next_letters, flows);
    }
  }
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 167409079868000);
  }
}
