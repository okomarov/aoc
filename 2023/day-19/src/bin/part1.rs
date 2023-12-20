use std::collections::{HashMap, VecDeque};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}
#[derive(Debug)]
struct Cond {
  letter: char,
  is_less: bool,
  than: usize,
  next: String,
  is_terminal: bool,
}

fn parse_input(input: &str) -> (Vec<HashMap<char, usize>>, HashMap<String, VecDeque<Cond>>) {
  let sections: Vec<&str> = input.split("\n\n").collect();
  let parts: Vec<HashMap<char, usize>> = sections[1]
    .lines()
    .map(|line| {
      line
        .trim_matches(|c| c == '{' || c == '}')
        .split(",")
        .filter_map(|part| {
          let items: Vec<&str> = part.split(['=', '}']).collect();
          // println!("{:?}", items);
          if items.len() == 2 {
            Some((
              items[0].chars().next().unwrap(),
              items[1].parse::<usize>().unwrap(),
            ))
          } else {
            None
          }
        })
        .collect()
    })
    .collect();

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
      letter: 'A',
      is_less: false,
      than: 0,
      next: ' '.to_string(),
      is_terminal: true,
    }]),
  );

  flows.insert(
    'R'.to_string(),
    VecDeque::from([Cond {
      letter: 'R',
      is_less: false,
      than: 0,
      next: ' '.to_string(),
      is_terminal: true,
    }]),
  );

  (parts, flows)
}

fn parse_condition(s: &str) -> Cond {
  let parts: Vec<&str> = s.split(':').collect();
  if parts.len() >= 2 {
    Cond {
      letter: parts[0].chars().nth(0).unwrap(),
      is_less: parts[0].chars().nth(1).unwrap() == '<',
      than: parts[0].get(2..).unwrap().parse::<usize>().unwrap(),
      next: parts[1].to_string(),
      is_terminal: false,
    }
  } else {
    Cond {
      letter: ' ',
      is_less: false,
      than: 0,
      next: s.to_string(),
      is_terminal: true,
    }
  }
}

fn part1(input: &str) -> usize {
  let (parts, flows) = parse_input(input);

  // println!("{:?}", parts);
  println!("{:?}", flows);

  let mut ans = 0;
  'next_part: for letters in parts.iter() {
    println!("part {:?}", letters);
    let mut run_flows = VecDeque::from([flows.get("in").unwrap()]);
    'next_flow: while let Some(flow) = run_flows.pop_front() {
      println!("flow {:?}", flow);
      for cond in flow.iter() {
        println!("cond {:?}", cond);

        if cond.is_terminal {
          if cond.letter == 'A' {
            println!("accepted");
            ans += letters.values().sum::<usize>();
            continue 'next_part;
          } else if cond.letter == 'R' {
            println!("rejected");
          } else {
            println!("next");
            run_flows.push_back(flows.get(&cond.next).unwrap())
          }
        } else if (cond.is_less && letters.get(&cond.letter).unwrap() < &cond.than)
          || (!cond.is_less && letters.get(&cond.letter).unwrap() > &cond.than)
        {
          println!("condition met");
          run_flows.push_back(flows.get(&cond.next).unwrap());
          continue 'next_flow;
        }
      }
    }
  }

  ans
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
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 19114);
  }
}
