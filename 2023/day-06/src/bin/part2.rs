fn main() {
  let input = include_str!("./input2.txt");
  let output = part2(input);
  dbg!(output);
}

fn part2(input: &str) -> u64 {
  let lines = input.lines().collect::<Vec<&str>>();
  let time = lines[0]
    .split_whitespace()
    .skip(1)
    .map(|x| x.parse::<u64>().expect("Number"));

  let dist = lines[1]
    .split_whitespace()
    .skip(1)
    .map(|x| x.parse::<u64>().expect("Number"))
    .collect::<Vec<_>>();

  time
    .enumerate()
    .map(|(i, T)| {
      let sq = ((T.pow(2) - 4 * dist[i]) as f64).sqrt();
      let mut ub = 0.5 * (T as f64 + sq);
      let mut lb = 0.5 * (T as f64 - sq);

      if ub % 1.0 == 0.0 {
        ub -= 1.0;
      }
      if lb % 1.0 == 0.0 {
        lb += 1.0;
      }
      println!("{lb}-{ub}");
      return (ub.floor() - lb.ceil()) as u64 + 1;
    })
    .product()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "Time:      71530
Distance:  940200";
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 71503);
  }
}
