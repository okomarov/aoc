fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part2(input);

  dbg!(output1);
}

#[derive(Default, Debug, Copy, Clone)]
struct C {
  x: f64,
  y: f64,
  z: f64,
  tx: f64,
  ty: f64,
  tz: f64,
}

impl C {
  fn from_origin(&self) -> u64 {
    (self.x as u64).pow(2) + (self.y as u64).pow(2) + (self.z as u64).pow(2)
  }
}

fn parse_input(input: &str) -> Vec<C> {
  input
    .lines()
    .map(|line| {
      let parts: Vec<&str> = line.split('@').collect();
      let point: Vec<f64> = parts[0]
        .split(',')
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect();
      let tel: Vec<f64> = parts[1]
        .split(',')
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect();

      C {
        x: point[0],
        y: point[1],
        z: point[2],
        tx: tel[0],
        ty: tel[1],
        tz: tel[2],
      }
    })
    .collect()
}

fn cross_product(a: &C, b: &C, c: &C) -> C {
  let ab = C {
    x: b.x - a.x,
    y: b.y - a.y,
    z: b.z - a.z,
    ..Default::default()
  };
  let ac = C {
    x: c.x - a.x,
    y: c.y - a.y,
    z: c.z - a.z,
    ..Default::default()
  };

  C {
    x: ab.y * ac.z - ab.z * ac.y,
    y: ab.z * ac.x - ab.x * ac.z,
    z: ab.x * ac.y - ab.y * ac.x,
    ..Default::default()
  }
}

fn get_point(point: &C, n: usize) -> C {
  C {
    x: point.x + point.tx * n as f64,
    y: point.y + point.ty * n as f64,
    z: point.z + point.tz * n as f64,
    ..Default::default()
  }
}

fn part2(input: &str) -> f64 {
  let stones = parse_input(input);
  // stones.sort_by_key(|stone| stone.from_origin());

  let ub = 100;
  let n = stones.len();

  for i in 0..n {
    for j in i + 1..n {
      for k in j + 1..n {
        let mut a = get_point(&stones[i], 1);
        let mut b = get_point(&stones[j], 2);
        let mut c = get_point(&stones[k], 3);

        let mut prev_2 = cross_product(&a, &b, &c);
        let mut skipped_t2 = true;

        println!("{i} {j} {k}");
        for t0 in 1..ub {
          for t1 in 1..ub {
            for t2 in 1..ub {
              if t2 == t1 || t2 == t0 || t0 == t1 {
                continue;
              }

              a = get_point(&stones[i], t0);
              b = get_point(&stones[j], t1);
              c = get_point(&stones[k], t2);

              let current = cross_product(&a, &b, &c);

              if skipped_t2 {
                prev_2 = current;
                skipped_t2 = false;
              }

              // println!(
              //   "({t0},{t1},{t2}) - {:?} - {:?}, {:?}, {:?}",
              //   current.x.abs() + current.y.abs() + current.z.abs(),
              //   current.x,
              //   current.y,
              //   current.z
              // );
              if current.x != 0.0 || current.y != 0.0 || current.z != 0.0 {
                if current.x.abs() + current.y.abs() + current.z.abs()
                  > prev_2.x.abs() + prev_2.y.abs() + prev_2.z.abs()
                {
                  skipped_t2 = true;
                  break;
                }
                prev_2 = current;
                continue;
              }

              println!("{t0} {t1} {t2}\n{:?}\n{:?}\n{:?}", a, b, c);
              if t1 < t2 {
                let dt = (t2 - t1) as f64;
                let start = C {
                  x: a.x - (b.x - a.x) / dt * t1 as f64,
                  y: a.y - (b.y - a.y) / dt * t1 as f64,
                  z: a.z - (b.z - a.z) / dt * t1 as f64,
                  ..Default::default()
                };
                println!("start: {:?}", start);
                return start.x + start.y + start.z;
              } else {
                let dt = (t1 - t2) as f64;
                let start = C {
                  x: a.x - (a.x - b.x) / dt * t1 as f64,
                  y: a.y - (a.y - b.y) / dt * t1 as f64,
                  z: a.z - (a.z - b.z) / dt * t1 as f64,
                  ..Default::default()
                };
                println!("start: {:?}", start);
                return start.x + start.y + start.z;
              }
            }
          }
        }
      }
    }
  }

  0.0
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
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 47.0);
  }
}
