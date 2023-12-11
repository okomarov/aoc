use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("./input1.txt");
  let output = part2(input);
  dbg!(output);
}
#[derive(Debug)]
struct Node {
  pos: (u32, u32),
  from: (i32, i32),
  count: u32,
}

fn part2(input: &str) -> usize {
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

  let mut board: Vec<Vec<char>> = input
    .lines()
    .filter(|line| line.len() != 0)
    .map(|line| line.chars().collect())
    .collect();
  print_board(&board, None);

  let path = get_path(&board, &nesw, &direction);
  clean_junk(&mut board, &path);
  print_board(&board, None);

  let new_board = expand_board(&board);
  print_board(&new_board, None);

  let mut dots = collect_dots(&new_board);
  print_board(&new_board, Some(&dots));

  exclude_external(&new_board, &nesw, &mut dots);
  print_board(&new_board, Some(&dots));

  let new_dots = shrink_dots(&dots);
  print_board(&board, Some(&new_dots));

  new_dots.len()
}

fn get_path(
  board: &Vec<Vec<char>>,
  nesw: &[(i32, i32); 4],
  direction: &HashMap<(&(i32, i32), &char), &(i32, i32)>,
) -> HashSet<(u32, u32)> {
  let s = get_start(&board).expect("Start position to exist");
  let mut path: HashSet<(u32, u32)> = HashSet::from([s]);

  let mut queue: Vec<Node> = nesw
    .to_vec()
    .iter()
    .map(|dir| Node {
      from: *dir,
      pos: ((s.0 as i32 + dir.0) as u32, (s.1 as i32 + dir.1) as u32),
      count: 0,
    })
    .collect();

  while let Some(node) = queue.pop() {
    let new_pos_option: Option<&char> = board
      .get(node.pos.0 as usize)
      .and_then(|this_row| this_row.get(node.pos.1 as usize));

    if new_pos_option.is_none() {
      continue;
    }
    let new_pos = new_pos_option.unwrap();

    if new_pos == &'S' {
      break;
    }

    path.insert(node.pos);

    match direction.get(&(&node.from, new_pos)) {
      Some(dir) => queue.push(Node {
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
  path
}

fn get_start(board: &Vec<Vec<char>>) -> Result<(u32, u32), String> {
  for (row, line) in board.iter().enumerate() {
    for (col, c) in line.iter().enumerate() {
      if c == &'S' {
        return Ok((row as u32, col as u32));
      }
    }
  }

  Err("Could not find start".to_string())
}

fn exclude_external(
  board: &Vec<Vec<char>>,
  nesw: &[(i32, i32); 4],
  dots: &mut HashSet<(usize, usize)>,
) {
  let mut queue: Vec<(usize, usize)> = collect_edge_positions(&board);
  let mut seen: HashSet<(usize, usize)> = Default::default();
  while let Some(node) = queue.pop() {
    if seen.contains(&node) {
      continue;
    }
    seen.insert(node);
    let char_option: Option<&char> = board.get(node.0).and_then(|this_row| this_row.get(node.1));
    if char_option.is_none() {
      continue;
    }
    let c = char_option.unwrap();
    if *c == '.' {
      dots.remove(&node);
      nesw.to_vec().iter().for_each(|d| {
        queue.push((
          (node.0 as i32 + d.0) as usize,
          (node.1 as i32 + d.1) as usize,
        ))
      });
    }
  }
}

fn collect_edge_positions(board: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let mut edge_positions = Vec::new();
  let row_count = board.len();

  let col_count = board[0].len();

  // First and last rows
  for col in 0..col_count {
    edge_positions.push((0, col)); // First row
    edge_positions.push((row_count - 1, col)); // Last row
  }

  // First and last columns (excluding already added corners)
  for row in 1..row_count - 1 {
    edge_positions.push((row, 0)); // First column
    edge_positions.push((row, col_count - 1)); // Last column
  }

  edge_positions
}

fn clean_junk(board: &mut Vec<Vec<char>>, path: &HashSet<(u32, u32)>) {
  board.iter_mut().enumerate().for_each(|(row, l)| {
    l.iter_mut().enumerate().for_each(|(col, c)| {
      if !path.contains(&(row as u32, col as u32)) {
        *c = '.'
      }
    })
  });
}

fn expand_board(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut new_board: Vec<Vec<char>> = Vec::with_capacity(board.len() * 2);

  for row in board {
    let mut first_row: Vec<char> = Vec::with_capacity(row.len() * 2);
    let mut second_row: Vec<char> = Vec::with_capacity(row.len() * 2);
    for &c in row {
      first_row.push(c);
      if c == '|' || c == '7' || c == 'J' || c == '.' {
        first_row.push('.');
      } else {
        first_row.push('-');
      }
    }

    for &cc in &first_row {
      if cc == '-' || cc == 'J' || cc == 'L' || cc == '.' {
        second_row.push('.')
      } else {
        second_row.push('|')
      }
    }

    new_board.push(first_row);
    new_board.push(second_row);
  }

  new_board
}

fn collect_dots(board: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
  let mut dots = HashSet::new();

  for (row, line) in board.iter().enumerate() {
    for (col, &c) in line.iter().enumerate() {
      if c == '.' {
        dots.insert((row, col));
      }
    }
  }

  dots
}

fn shrink_dots(dots: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
  let mut new_dots = HashSet::new();
  for &(row, col) in dots.iter() {
    if row % 2 == 0 && col % 2 == 0 {
      new_dots.insert(((row / 2) as usize, (col / 2) as usize));
    }
  }
  new_dots
}

fn print_board(board: &Vec<Vec<char>>, dots: Option<&HashSet<(usize, usize)>>) {
  board.iter().enumerate().for_each(|(row, line)| {
    line.iter().enumerate().for_each(|(col, &c)| {
      if dots.map_or(false, |d| d.contains(&(row, col))) {
        print!("I");
      } else {
        print!("{c}");
      }
    });
    println!();
  });
  println!();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 4);
  }

  #[test]
  fn it_works2() {
    let input = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 8);
  }
  #[test]
  fn it_works3() {
    let input = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 10);
  }
}
