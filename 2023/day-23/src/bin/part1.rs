use std::{cmp::max, collections::HashSet};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}
#[derive(Debug)]
struct Node {
  key: Key,
  count: usize,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Key {
  pos: (usize, usize),
  dir: (i32, i32),
  path: usize,
}

fn part1(input: &str) -> usize {
  let nesw: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
  let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let len = board.len();
  let mut ans = 0;

  let mut queue: Vec<Node> = vec![Node {
    key: Key {
      pos: (0, 1),
      path: 0,
      dir: (1, 0),
    },
    count: 0,
  }];
  let mut seen: HashSet<Key> = HashSet::new();

  while let Some(node) = queue.pop() {
    // println!("{:?}", node);
    if seen.contains(&node.key) {
      continue;
    }
    if node.key.pos == (len - 1, len - 2) {
      ans = max(ans, node.count);
      continue;
    }

    seen.insert(node.key);

    let mut index = 0;
    for (dr, dc) in nesw.iter() {
      let (new_r, new_c) = (node.key.pos.0 as i32 + dr, node.key.pos.1 as i32 + dc);
      if new_r < 0
        || new_r as usize >= len
        || new_c < 0
        || new_c as usize >= len
        || (new_r, new_c)
          == (
            node.key.pos.0 as i32 - node.key.dir.0,
            node.key.pos.1 as i32 - node.key.dir.1,
          )
      {
        continue;
      } else {
        let c = board
          .get(new_r as usize)
          .and_then(|line| line.get(new_c as usize))
          .unwrap();
        if c == &'.'
          || (c == &'>' && (dr, dc) == (&0, &1))
          || (c == &'^' && (dr, dc) == (&-1, &0))
          || (c == &'<' && (dr, dc) == (&0, &-1))
          || (c == &'v' && (dr, dc) == (&1, &0))
        {
          queue.push(Node {
            key: Key {
              pos: (new_r as usize, new_c as usize),
              dir: (*dr, *dc),
              path: node.key.path.clone() + index,
            },
            count: node.count + 1,
          })
        }
        index += 1;
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
    let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 16);
  }
}
