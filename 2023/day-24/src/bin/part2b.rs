fn main() {
  let input = include_str!("./input2.txt");
  let output1 = part2(input);

  dbg!(output1);
}

#[derive(Default, Debug, Copy, Clone)]
struct C {
  x: f64,
  y: f64,
  z: f64,
  vx: f64,
  vy: f64,
  vz: f64,
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
        vx: tel[0],
        vy: tel[1],
        vz: tel[2],
      }
    })
    .collect()
}

fn cross_product_dist(stones: &Vec<C>, times: &[i64; 3]) -> f64 {
  let a = get_point(&stones[0], times[0]);
  let b = get_point(&stones[1], times[1]);
  let c = get_point(&stones[2], times[2]);

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

fn get_point(point: &C, n: i64) -> C {
  C {
    x: point.x + point.vx * n as f64,
    y: point.y + point.vy * n as f64,
    z: point.z + point.vz * n as f64,
    ..Default::default()
  }
}

fn part2(input: &str) -> f64 {
  let stones = parse_input(input);
  // [-65219346032, -74544243727, 8257916505];
  let mut times: [i64; 3] = [791056720687, 501548876863, 913402098647];
  let mut variations: [f64; 6] = [f64::MAX, f64::MAX, f64::MAX, f64::MAX, f64::MAX, f64::MAX];
  let mut base = cross_product_dist(&stones, &times);

  loop {
    let mut changed_times_pos;
    // let mut changed_times_neg;
    for i in 0..=2 {
      changed_times_pos = times;
      changed_times_pos[i] += 1;
      variations[i] = cross_product_dist(&stones, &changed_times_pos);

      // changed_times_neg = times;
      // changed_times_neg[i] -= 1;
      // variations[3 + i] = cross_product_dist(&stones, &changed_times_neg);
    }
    let df: Vec<f64> = variations.iter().map(|v| base - v).collect();
    let choice = df
      .iter()
      .enumerate()
      .max_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap())
      .unwrap();

    let k = if choice.0 > 2 { choice.0 - 3 } else { choice.0 };
    times[k] += if choice.0 > 2 { -1 } else { 1 };
    base = variations[choice.0];

    println!("{:?} - {:?}", times, base);
    if base < 1e-6 {
      break;
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
  #[test]
  fn it_works2() {
    let input = r#"207368289604003, 208550616378928, 178175262907962 @ 45, -58, 25
206500202846264, 256535196755432, 135051442506500 @ 47, -481, 383
202597289456237, 252168926544368, 139168135303676 @ 135, 43, 268"#;
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 47.0);
  }
}
