use std::collections::HashMap;

fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}
#[derive(Debug)]
struct Node {
  pos: (u32, u32),
  from: (i32, i32),
  count: u32,
}

fn part1(input: &str) -> u32 {
  let nesw: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
  let direction: HashMap<(&(i32, i32), &char), &(i32, i32)> = HashMap::from([
    ((&(-1, 0), &'|'), &(-1, 0)),
    ((&(-1, 0), &'F'), &(0, 1)),
    ((&(-1, 0), &'7'), &(0, -1)),
    ((&(0, 1), &'-'), &(0, 1)),
    ((&(0, 1), &'J'), &(-1, 0)),
    ((&(0, 1), &'7'), &(1, 0)),
    ((&(1, 0), &'|'), &(1, 0)),
    ((&(1, 0), &'J'), &(0, -1)),
    ((&(1, 0), &'L'), &(0, 1)),
    ((&(0, -1), &'-'), &(0, -1)),
    ((&(0, -1), &'F'), &(1, 0)),
    ((&(0, -1), &'L'), &(-1, 0)),
  ]);

  let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

  let mut s = (0, 0);
  'outer: for (row, line) in board.iter().enumerate() {
    for (col, c) in line.iter().enumerate() {
      if c == &'S' {
        s = (row, col);
        break 'outer;
      }
    }
  }

  let mut seen: Vec<Node> = nesw
    .to_vec()
    .iter()
    .map(|dir| Node {
      from: *dir,
      pos: ((s.0 as i32 + dir.0) as u32, (s.1 as i32 + dir.1) as u32),
      count: 0,
    })
    .collect();

  while let Some(node) = seen.pop() {
    let new_pos_option: Option<&char> = board
      .get(node.pos.0 as usize)
      .and_then(|this_row| this_row.get(node.pos.1 as usize));

    if new_pos_option.is_none() {
      continue;
    }
    let new_pos = new_pos_option.unwrap();
    if new_pos == &'S' {
      return (node.count as f64 / 2.0).ceil() as u32;
    }
    match direction.get(&(&node.from, new_pos)) {
      Some(dir) => seen.push(Node {
        from: **dir,
        count: node.count + 1,
        pos: (
          (node.pos.0 as i32 + dir.0) as u32,
          (node.pos.1 as i32 + dir.1) as u32,
        ),
      }),
      None => continue,
    }
  }

  1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "
.....
.S-7.
.|.|.
.L-J.
.....";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 4);
  }
}
