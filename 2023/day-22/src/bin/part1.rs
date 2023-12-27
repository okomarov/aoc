use std::collections::{HashMap, HashSet};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}
#[derive(Debug)]
struct R {
  name: usize,
  x1: usize,
  y1: usize,
  z1: usize,
  x2: usize,
  y2: usize,
  z2: usize,
}

fn parse_input(input: &str) -> Vec<R> {
  input
    .lines()
    .enumerate()
    .map(|(i, line)| {
      let parts: Vec<&str> = line.split("~").collect();
      let point1: Vec<usize> = parts[0]
        .split(",")
        .map(|item| item.parse::<usize>().unwrap())
        .collect();

      let point2: Vec<usize> = parts[1]
        .split(",")
        .map(|item| item.parse::<usize>().unwrap())
        .collect();

      return R {
        name: i + 1,
        x1: point1[0],
        y1: point1[1],
        z1: point1[2],
        x2: point2[0],
        y2: point2[1],
        z2: point2[2],
      };
    })
    .collect()
}

fn intersect(b1: &R, b2: &R) -> bool {
  !(b1.x2 < b2.x1 || b2.x2 < b1.x1 || b1.y2 < b2.y1 || b2.y2 < b1.y1)
}

fn settle_bricks(bricks: &mut Vec<R>) {
  bricks.sort_by_key(|k| k.z1);

  for current in 0..bricks.len() {
    let mut settled = false;
    for other in (0..current).rev() {
      if intersect(&bricks[current], &bricks[other]) {
        let dz = bricks[current].z2 - bricks[current].z1;
        bricks[current].z1 = bricks[other].z2 + 1;
        bricks[current].z2 = bricks[current].z1 + dz;
        settled = true;
        break;
      }
    }
    if !settled {
      let dz = bricks[current].z2 - bricks[current].z1;
      bricks[current].z1 = 1;
      bricks[current].z2 = 1 + dz;
    }
  }
  bricks.sort_by_key(|k| k.z1);
}

fn settle_bricks2(bricks: &mut Vec<R>) {
  let mut moved_any = true;
  while moved_any {
    moved_any = false;
    for current in 0..bricks.len() {
      let mut intersected = false;
      for other in 0..bricks.len() {
        if current == other {
          continue;
        } else if bricks[current].z1 == bricks[other].z2 + 1
          && intersect(&bricks[current], &bricks[other])
        {
          intersected = true;
          break;
        }
      }
      if !intersected && bricks[current].z1 > 1 {
        bricks[current].z1 -= 1;
        bricks[current].z2 -= 1;
        moved_any = true;
      }
    }
  }
}

fn part1(input: &str) -> usize {
  let mut bricks = parse_input(input);
  settle_bricks2(&mut bricks);

  bricks.iter().for_each(|b| println!("{:?}", b));

  let mut brick_map: HashMap<usize, Vec<usize>> = HashMap::new();
  for current in 0..bricks.len() {
    for other in 0..bricks.len() {
      if bricks[other].z2 == (bricks[current].z1 - 1) && intersect(&bricks[current], &bricks[other])
      {
        brick_map
          .entry(bricks[current].name)
          .and_modify(|names| names.push(bricks[other].name))
          .or_insert(vec![bricks[other].name]);
      }
    }
  }

  println!(
    "{:?}{:?}",
    bricks.len(),
    HashSet::from((1..=bricks.len()).collect::<HashSet<usize>>())
      .difference(&HashSet::from(
        brick_map.keys().copied().collect::<HashSet<usize>>()
      ))
      .count()
  );

  let single_support: HashSet<usize> = brick_map
    .values()
    .filter_map(|v| if v.len() == 1 { Some(v) } else { None })
    .flatten()
    .copied()
    .collect();

  HashSet::from((1..=bricks.len()).collect::<HashSet<usize>>())
    .difference(&single_support)
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 5);
  }
}
