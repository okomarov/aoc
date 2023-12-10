fn main() {
  let input = include_str!("./input1.txt");
  let output = part2(input);
  dbg!(output);
}

fn next_val(nums: &Vec<i64>) -> i64 {
  let diffs: Vec<i64> = nums.windows(2).map(|w| w[1] - w[0]).collect();

  if diffs.iter().all(|v| &diffs[0] == v) {
    return diffs[0];
  }
  diffs[0] - next_val(&diffs)
}

fn part2(input: &str) -> i64 {
  input
    .lines()
    .map(|line| {
      line
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("Number"))
        .collect::<Vec<i64>>()
    })
    .map(|nums| {
      let val = next_val(&nums);
      return nums[0] - val;
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let score = part2(input);
    print!("{}", score);
    assert_eq!(score, 2);
  }
}
