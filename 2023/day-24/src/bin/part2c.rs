use nalgebra::Vector3;

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
  fn dist(&self, other: &C) -> u64 {
    ((self.x as u64).pow(2) - (other.x as u64).pow(2))
      + ((self.y as u64).pow(2) - (other.y as u64).pow(2))
      + ((self.z as u64).pow(2) - (other.z as u64).pow(2))
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

fn cross_product_dist(a: &C, b: &C, c: &C) -> f64 {
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
  (ab.y * ac.z - ab.z * ac.y).powi(2)
    + (ab.z * ac.x - ab.x * ac.z).powi(2)
    + (ab.x * ac.y - ab.y * ac.x).powi(2)
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

  let n = stones.len();
  let mut min = f64::MAX;
  for i in 0..n {
    for j in i + 1..n {
      for k in j + 1..n {
        let a = get_point(&stones[i], 0);
        let b = get_point(&stones[j], 0);
        let c = get_point(&stones[k], 0);
        let dist = cross_product_dist(&a, &b, &c);
        if dist < min {
          println!("({i},{j},{k}) - {dist}");
          min = dist;
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
