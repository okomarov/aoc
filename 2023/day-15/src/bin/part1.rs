fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

fn hash(s: &str) -> usize {
  let mut val = 0;
  for c in s.chars() {
    val += (c as u8) as usize;
    val *= 17;
    val %= 256;
  }
  val
}

fn part1(input: &str) -> usize {
  let steps: Vec<&str> = input.split(",").collect();
  steps.iter().fold(0, |acc, step| acc + hash(step))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 1320);
  }
}
