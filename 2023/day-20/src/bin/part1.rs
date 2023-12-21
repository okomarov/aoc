use std::collections::{HashMap, VecDeque};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}
// initially off.
// Ignores high pulses.
// Low pulse, it flips state:
//  1. off => on +  high pulse.
//  2. on => off + low pulse.
#[derive(Debug)]
struct Flip<'a> {
  state: bool,
  next: Vec<&'a str>,
}

// Remembers connected input modules;
// Initially remembers a low pulse for each input.
// First updates its memory, then sends pulse
// If all high => low pulse else high
#[derive(Debug)]
struct Con<'a> {
  memory: HashMap<&'a str, bool>,
  next: Vec<&'a str>,
}

#[derive(Debug)]
struct Broadcaster<'a> {
  next: Vec<&'a str>,
}

#[derive(Debug)]
enum Module<'a> {
  Flip(Flip<'a>),
  Con(Con<'a>),
  Broadcaster(Broadcaster<'a>),
}

fn parse_input<'a>(input: &str) -> HashMap<&str, Module> {
  let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
  let mut modules: HashMap<&str, Module> = HashMap::new();

  for line in input.lines() {
    let parts: Vec<&str> = line.split(" -> ").collect();
    let connected: Vec<&str> = parts[1].split(",").map(|s| s.trim()).collect();
    let origin = &parts[0][1..];

    // println!("{line} {origin} {:?}", connected);

    if parts[0].starts_with("broadcaster") {
      rules.insert("broadcaster", connected.clone());

      modules.insert(
        "broadcaster",
        Module::Broadcaster(Broadcaster {
          next: connected.iter().copied().collect(),
        }),
      );
    } else if parts[0].starts_with("&") {
      rules.insert(origin, connected.clone());

      modules.insert(
        origin,
        Module::Con(Con {
          memory: HashMap::new(),
          next: connected.iter().copied().collect(),
        }),
      );
    } else {
      rules.insert(origin, connected.clone());

      modules.insert(
        origin,
        Module::Flip(Flip {
          state: false,
          next: connected.iter().copied().collect(),
        }),
      );
    }
  }

  for (origin, connected) in rules.iter_mut() {
    for con in connected.iter_mut() {
      match modules.get_mut(con) {
        Some(Module::Con(c)) => c.memory.insert(origin, false),
        _ => continue,
      };
    }
  }

  modules
}

fn part1(input: &str) -> usize {
  let mut modules = parse_input(input);
  let (mut low, mut high) = (0, 0);

  // println!("{:?}", modules);

  // Remembers connected input modules;
  // Initially remembers a low pulse for each input.
  // First updates its memory, then sends pulse
  // If all high => low pulse else high

  // initially off.
  // Ignores high pulses.
  // Low pulse, it flips state:
  //  1. off => on +  high pulse.
  //  2. on => off + low pulse.

  for _ in 0..1000 {
    let mut queue: VecDeque<(&str, bool, &str)> =
      VecDeque::from([("broadcaster", false, "button")]);

    while let Some((current, signal, previous)) = queue.pop_front() {
      if signal {
        high += 1
      } else {
        low += 1
      };

      // println!("{current}, {signal}, {previous}");

      match modules.get_mut(current) {
        Some(Module::Broadcaster(c)) => c
          .next
          .iter()
          .for_each(|name| queue.push_back((name, false, current))),
        Some(Module::Flip(c)) => {
          if signal {
            continue;
          } else {
            c.next
              .iter()
              .for_each(|name| queue.push_back((name, !c.state, current)));
            c.state = !c.state;
          }
        }
        Some(Module::Con(c)) => {
          c.memory.entry(previous).and_modify(|v| *v = signal);
          if c.memory.values().all(|v| *v) {
            c.next
              .iter()
              .for_each(|name| queue.push_back((name, false, current)));
          } else {
            c.next
              .iter()
              .for_each(|name| queue.push_back((name, true, current)));
          }
        }
        None => {}
      }
    }
  }

  low * high
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 32000000);
  }
}
