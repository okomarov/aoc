use std::{
  cmp::max,
  collections::{HashMap, HashSet, VecDeque},
};

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

fn settle_bricks2(bricks: &mut Vec<R>) {
  bricks.sort_by_key(|k| k.z1);

  let mut moved_any = true;
  while moved_any {
    moved_any = false;
    for current in 0..bricks.len() {
      let mut intersected = false;
      let mut z = 0;
      for other in (0..current).rev() {
        if bricks[current].z1 == bricks[other].z2 + 1 && intersect(&bricks[current], &bricks[other])
        {
          intersected = true;
          break;
        } else if bricks[other].z2 < bricks[current].z1 - 1 {
          z = max(z, bricks[other].z2 + 1);
        }
      }
      if !intersected && bricks[current].z1 > 1 {
        let dz = bricks[current].z2 - bricks[current].z1;
        bricks[current].z1 = z;
        bricks[current].z2 = z + dz;
        moved_any = true;
      }
    }
  }
  bricks.sort_by_key(|k| k.z1);
}

fn part1(input: &str) -> usize {
  let mut bricks = parse_input(input);
  settle_bricks2(&mut bricks);

  // bricks.iter().for_each(|b| println!("{:?}", b));

  let mut supported: HashMap<usize, Vec<usize>> = HashMap::new();
  for current in 0..bricks.len() {
    for supported_by in 0..bricks.len() {
      if bricks[supported_by].z2 == (bricks[current].z1 - 1)
        && intersect(&bricks[current], &bricks[supported_by])
      {
        supported
          .entry(bricks[current].name)
          .and_modify(|names| names.push(bricks[supported_by].name))
          .or_insert(vec![bricks[supported_by].name]);
      }
    }
  }

  let single_support: HashSet<usize> = supported
    .values()
    .filter_map(|v| if v.len() == 1 { Some(v) } else { None })
    .flatten()
    .copied()
    .collect();

  let mut supports: HashMap<usize, Vec<usize>> = HashMap::new();
  for base in 0..bricks.len() {
    for other in 0..bricks.len() {
      if bricks[base].z2 == bricks[other].z1 - 1 && intersect(&bricks[base], &bricks[other]) {
        supports
          .entry(bricks[base].name)
          .and_modify(|names| names.push(bricks[other].name))
          .or_insert(vec![bricks[other].name]);
      }
    }
  }

  let mut ans = 0;
  for single in single_support.iter() {
    let start = supports.get(&single).unwrap();
    let mut removed: HashSet<usize> = HashSet::from([*single]);
    let mut queue: VecDeque<usize> = start.iter().cloned().collect();

    while let Some(brick) = queue.pop_front() {
      if !supported.contains_key(&brick) || removed.contains(&brick) {
        continue;
      }

      if supported
        .get(&brick)
        .unwrap()
        .iter()
        .all(|supported_by| removed.contains(supported_by))
      {
        ans += 1;
        removed.insert(brick);
        if supports.contains_key(&brick) {
          queue.extend(supports.get(&brick).unwrap());
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
