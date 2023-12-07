fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1(input: &str) -> i32 {
  let mut sum = 0;

  for line in input.lines() {
    let mut num = String::with_capacity(2);
    let mut last_digit = '0';
    for c in line.chars() {
      if num.len() == 0 && c.is_ascii_digit() {
        num.push(c);
        last_digit = c;
      } else if c.is_ascii_digit() {
        last_digit = c;
      }
    }
    num.push(last_digit);
    match num.parse::<i32>() {
      Ok(number) => sum += number,
      Err(e) => println!("Failed to parse: {}", e),
    }
  }
  return sum;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    assert_eq!(part1(input), 142);
  }
}
