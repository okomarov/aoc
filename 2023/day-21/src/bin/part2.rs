use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

fn hash_set(set: &HashSet<(i32, i32)>) -> u64 {
  let mut tuples: Vec<_> = set.iter().collect();
  tuples.sort_unstable(); // Sort the tuples

  let mut hasher = DefaultHasher::new();
  for tuple in tuples {
    let mut elem_hasher = DefaultHasher::new();
    tuple.hash(&mut elem_hasher);
    let elem_hash = elem_hasher.finish();

    // Combine the hashes in a deterministic way
    hasher.write_u64(elem_hash);
  }

  hasher.finish()
}

fn assign_outer(
  outer: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>,
  pos: &(i32, i32),
  board_pos: &(&i32, &i32),
  len: &i32,
) {
  let (r, c) = pos;
  let (board_r, board_c) = *board_pos;

  if *r < 0 {
    let pos = ((len - 1), *c);
    outer
      .entry((board_r - 1, *board_c))
      .and_modify(|temp| {
        temp.insert(pos);
      })
      .or_insert(HashSet::from([pos]));
  } else if r >= len {
    let pos = (0, *c);
    outer
      .entry((board_r + 1, *board_c))
      .and_modify(|temp| {
        temp.insert(pos);
      })
      .or_insert(HashSet::from([pos]));
  } else if 0 > *c {
    let pos = (*r, len - 1);
    outer
      .entry((*board_r, board_c - 1))
      .and_modify(|temp| {
        temp.insert(pos);
      })
      .or_insert(HashSet::from([pos]));
  } else if c >= len {
    let pos = (*r, 0);
    outer
      .entry((*board_r, board_c + 1))
      .and_modify(|temp| {
        temp.insert(pos);
      })
      .or_insert(HashSet::from([pos]));
  }
}
fn part1(input: &str) -> usize {
  let nesw: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
  let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let len = board.len() as i32;
  let board_mid = (len / 2) as i32;
  let mut now_all: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::from([(
    (0 as i32, 0 as i32),
    HashSet::from([(board_mid, board_mid)]),
  )]);

  let mut history: HashMap<u64, HashSet<(i32, i32)>> = HashMap::new();

  for i in 1..=1000 {
    let mut outer: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    for ((board_row, board_col), now) in now_all.iter_mut() {
      let mut temp: HashSet<(i32, i32)> = HashSet::new();
      let now_hash = hash_set(&now);
      if let Some(existing) = history.get(&now_hash) {
        temp = existing.clone();
      } else {
        for (row, col) in now.iter() {
          for (dr, dc) in nesw {
            let (new_r, new_c) = (row + dr, col + dc);

            // Store outer position
            if new_r < 0 || new_r >= len || new_c < 0 || new_c >= len {
              temp.insert((new_r, new_c));
            } else {
              let val = board
                .get(new_r as usize)
                .and_then(|line| line.get(new_c as usize))
                .unwrap();

              if val == &'.' || val == &'S' {
                temp.insert((new_r, new_c));
              }
            }
          }
        }
        history.insert(now_hash, temp.clone());
      }

      // Outer
      temp
        .iter()
        .filter(|(r, c)| *r < 0 || *r >= len || *c < 0 || *c >= len)
        .for_each(|pos| assign_outer(&mut outer, &pos, &(board_row, board_col), &len));

      *now = temp
        .iter()
        .filter(|(r, c)| *r >= 0 && *r < len && *c >= 0 && *c < len)
        .copied()
        .collect();
    }
    for (key, value) in outer.iter() {
      now_all
        .entry(*key)
        .and_modify(|s| s.extend(value))
        .or_insert_with(|| {
          println!("{i} {:?}", key);
          // print_board(&board, Some(value));
          return value.clone();
        });
    }
    // println!(
    //   "{i} {:?}",
    //   now_all.values().map(|now| now.len()).sum::<usize>()
    // );
  }

  now_all.values().map(|now| now.len()).sum()
}

fn print_board(board: &Vec<Vec<char>>, filled: Option<&HashSet<(i32, i32)>>) {
  board.iter().enumerate().for_each(|(row, line)| {
    line.iter().enumerate().for_each(|(col, &c)| {
      if filled.map_or(false, |d| d.contains(&(row as i32, col as i32))) {
        print!("O");
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
    let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 16);
  }
}
