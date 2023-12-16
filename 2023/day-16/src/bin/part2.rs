use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part2(input);

  dbg!(output1);
}

#[derive(Debug, Clone, Copy)]
struct Node {
  pos: (usize, usize),
  from: (i32, i32),
  count: usize,
}

fn part2(input: &str) -> usize {
  let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut starts: Vec<Node> = vec![];
  for r in 0..board.len() {
    starts.push(Node {
      from: (0, 1),
      pos: (r, 0),
      count: 1,
    })
  }

  for r in 0..board.len() {
    starts.push(Node {
      from: (0, -1),
      pos: (r, board[0].len() - 1),
      count: 1,
    })
  }

  for c in 0..board[0].len() {
    starts.push(Node {
      from: (1, 0),
      pos: (0, c),
      count: 1,
    })
  }

  for c in 0..board[0].len() {
    starts.push(Node {
      from: (-1, 0),
      pos: (board.len() - 1, c),
      count: 1,
    })
  }

  starts
    .iter()
    .map(|start| beam(&board, start))
    .max()
    .unwrap()
}

fn beam(board: &Vec<Vec<char>>, start: &Node) -> usize {
  let dirs: HashMap<((i32, i32), char), Vec<(i32, i32)>> = HashMap::from([
    (((1, 0), '|'), vec![(1, 0)]),
    (((-1, 0), '|'), vec![(-1, 0)]),
    (((0, -1), '|'), vec![(-1, 0), (1, 0)]),
    (((0, 1), '|'), vec![(-1, 0), (1, 0)]),
    (((1, 0), '-'), vec![(0, -1), (0, 1)]),
    (((-1, 0), '-'), vec![(0, -1), (0, 1)]),
    (((0, -1), '-'), vec![(0, -1)]),
    (((0, 1), '-'), vec![(0, 1)]),
    (((1, 0), '/'), vec![(0, -1)]),
    (((-1, 0), '/'), vec![(0, 1)]),
    (((0, -1), '/'), vec![(1, 0)]),
    (((0, 1), '/'), vec![(-1, 0)]),
    (((1, 0), '\\'), vec![(0, 1)]),
    (((-1, 0), '\\'), vec![(0, -1)]),
    (((0, -1), '\\'), vec![(-1, 0)]),
    (((0, 1), '\\'), vec![(1, 0)]),
  ]);

  let mut seen: HashSet<((i32, i32), (usize, usize))> = HashSet::new();
  let mut queue: Vec<Node> = vec![*start];

  while let Some(node) = queue.pop() {
    // println!("{:?}", node);
    if seen.contains(&(node.from, node.pos)) {
      continue;
    }

    // Handle out of bounds
    let item_option: Option<&char> = board
      .get(node.pos.0)
      .and_then(|this_row| this_row.get(node.pos.1));
    if item_option.is_none() {
      continue;
    }
    seen.insert((node.from, node.pos));

    let item = item_option.unwrap();
    if item == &'.' {
      let new_pos = (
        node.pos.0 as i32 + node.from.0,
        node.pos.1 as i32 + node.from.1,
      );
      if new_pos.0 < 0 || new_pos.1 < 0 {
        continue;
      }
      queue.push(Node {
        from: node.from,
        pos: (
          (node.pos.0 as i32 + node.from.0) as usize,
          (node.pos.1 as i32 + node.from.1) as usize,
        ),
        count: node.count + 1,
      })
    } else {
      for dir in dirs.get(&(node.from, *item)).expect("Dir") {
        let new_pos = (node.pos.0 as i32 + dir.0, node.pos.1 as i32 + dir.1);
        if new_pos.0 < 0 || new_pos.1 < 0 {
          continue;
        }
        queue.push(Node {
          from: *dir,
          pos: (new_pos.0 as usize, new_pos.1 as usize),
          count: node.count + 1,
        })
      }
    }
  }

  let unique: HashSet<(usize, usize)> = seen.into_iter().map(|item| item.1).collect();
  println!("{:?} {:?}", start, unique.len());
  unique.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 51);
  }
}
