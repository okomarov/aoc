use std::{
  cmp::max,
  collections::{HashMap, HashSet},
};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}
#[derive(Debug, Copy, Clone)]
struct Node {
  pos: (usize, usize),
  dir: (i32, i32),
  start: (usize, usize),
  count: usize,
}

fn walk(
  start: Node,
  seen: &mut HashSet<(usize, usize)>,
  len: &usize,
  board: &Vec<Vec<char>>,
  walked: &mut HashMap<((usize, usize), (i32, i32)), usize>,
  max_count: &mut usize,
) -> usize {
  let nesw: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
  let mut queue: Vec<Node> = vec![start];

  while let Some(node) = queue.pop() {
    // println!("{:?}", node);
    if seen.contains(&node.pos) {
      continue;
    }
    seen.insert(node.pos);

    if node.pos == (len - 1, len - 2) {
      // print_board(board, &seen);
      *max_count = max(*max_count, node.count);
      println!("{:?}", max_count);

      continue;
    }

    let mut candidates = vec![];
    for (dr, dc) in nesw.iter() {
      let (new_r, new_c) = (node.pos.0 as i32 + dr, node.pos.1 as i32 + dc);
      if new_r < 0 || new_r as usize >= *len || new_c < 0 || new_c as usize >= *len {
        continue;
      } else {
        // println!("{new_r}, {new_c}");
        let c = board
          .get(new_r as usize)
          .and_then(|line| line.get(new_c as usize))
          .unwrap();
        if c != &'#' && !seen.contains(&(new_r as usize, new_c as usize)) {
          candidates.push(((new_r as usize, new_c as usize), (*dr, *dc)))
        }
      }
    }
    if candidates.len() == 1 {
      queue.push(Node {
        pos: candidates[0].0,
        dir: candidates[0].1,
        start: node.start,
        count: node.count + 1,
      })
    } else if candidates.len() > 1 {
      let count = candidates
        .iter()
        .map(|turn| match walked.get(&(node.pos, node.dir)) {
          Some(total_steps) => *total_steps,
          None => walk(
            Node {
              pos: turn.0,
              start: turn.0,
              dir: turn.1,
              count: node.count + 1,
            },
            &mut seen.clone(),
            &len,
            &board,
            walked,
            max_count,
          ),
        })
        .max()
        .unwrap();
      if count > *max_count {
        *max_count = count;
        walked.insert((node.start, node.dir), count);
      }
    }
  }
  *max_count
}

// fn print_board(board: &Vec<Vec<char>>, seen: &HashSet<(usize, usize)>) {
//   board.iter().enumerate().for_each(|(row, line)| {
//     line.iter().enumerate().for_each(|(col, &c)| {
//       if seen.contains(&(row, col)) {
//         print!("O");
//       } else {
//         print!("{c}");
//       }
//     });
//     println!();
//   });
//   println!();
// }

fn part1(input: &str) -> usize {
  let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let len = board.len();
  let mut walked: HashMap<((usize, usize), (i32, i32)), usize> = HashMap::new();

  let mut seen: HashSet<(usize, usize)> = HashSet::new();

  walk(
    Node {
      pos: (0, 1),
      start: (0, 1),
      dir: (1, 0),
      count: 0,
    },
    &mut seen,
    &len,
    &board,
    &mut walked,
    &mut 0,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = r#"#.##########
#....######
####.######
####.######
####.######
####.######
####.######
###.......#
###.#####.#
###v#...#.#
###.>.#...#
#########.#"#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 23);
  }
  #[test]
  fn it_works2() {
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
    assert_eq!(score, 154);
  }
  #[test]
  fn it_works3() {
    let input = r#"#.########
#.#...####
#.#.#.####
#.#.#.####
#.#.#.####
#...#.####
###......#
###.##.#.#
###......#
########.#"#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 32);
  }
}
