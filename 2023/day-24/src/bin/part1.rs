fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

struct C {
  x: f64,
  y: f64,
  vx: f64,
  vy: f64,
  m: f64,
  c: f64,
}

fn intersect(l1: &C, l2: &C) -> (f64, f64) {
  let x = (l2.c - l1.c) / (l1.m - l2.m);
  let y = x * l1.m + l1.c;

  return (x, y);
}

fn is_future(p: &(f64, f64), l: &C) -> bool {
  ((l.vx > 0.0 && l.x < p.0) || (l.vx < 0.0 && l.x > p.0))
    && ((l.vy > 0.0 && l.y < p.1) || (l.vy < 0.0 && l.y > p.1))
}

fn part1(input: &str) -> usize {
  let mut ans = 0;
  let lb = 7.0;
  let ub = 27.0;
  let lb = 200000000000000.0;
  let ub = 400000000000000.0;
  let lines: Vec<C> = input
    .lines()
    .map(|line| {
      let parts: Vec<&str> = line.split('@').collect();
      let point: Vec<f64> = parts[0]
        .split(',')
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect();
      let vel: Vec<f64> = parts[1]
        .split(',')
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect();

      return C {
        x: point[0],
        y: point[1],
        vx: vel[0],
        vy: vel[1],
        m: vel[1] / vel[0],
        c: point[1] - (vel[1] / vel[0]) * point[0],
      };
    })
    .collect();

  for (index, l1) in lines.iter().enumerate() {
    for l2 in lines.iter().skip(index + 1) {
      if l1.m == l2.m {
        continue;
      }
      let (x, y) = intersect(l1, l2);

      if x >= lb
        && x <= ub
        && y >= lb
        && y <= ub
        && is_future(&(x, y), &l1)
        && is_future(&(x, y), &l2)
      {
        println!(
          "({x}, {y}) - ({:?} @ {:?}, {:?} @ {:?})",
          l1.x, l1.vx, l1.y, l1.vy
        );
        println!(
          "({x}, {y}) - ({:?} @ {:?}, {:?} @ {:?})",
          l2.x, l2.vx, l2.y, l2.vy
        );

        ans += 1
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
    let input = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 2);
  }
}
