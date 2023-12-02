use std::collections::HashMap;

fn main() {
  let input = include_str!("./input1.txt");
  let output = part2(input);
  dbg!(output);
}

fn get_str_digits_map() -> HashMap<&'static str, char> {
  let str_digits: HashMap<&str, char> = HashMap::from([
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
  ]);

  return str_digits;
}

fn match_first_str_digit(s: &str) -> (char, usize) {
  let str_digits: HashMap<&str, char> = get_str_digits_map();

  let mut min_pos = usize::MAX;
  let mut num = &'0';
  for (key, val) in str_digits.iter() {
    if let Some(pos) = s.find(key) {
      if pos <= min_pos {
        min_pos = pos;
        num = val;
      }
    }
  }

  return (*num, min_pos);
}

fn match_last_str_digit(s: &str) -> (char, usize) {
  let str_digits: HashMap<&str, char> = get_str_digits_map();

  let mut max_pos = 0;
  let mut num = &'0';
  for (key, val) in str_digits.iter() {
    if let Some(pos) = s.rfind(key) {
      if pos >= max_pos {
        max_pos = pos;
        num = val;
      }
    }
  }

  return (*num, max_pos);
}

fn match_first_char_digit(s: &str) -> (char, usize) {
  let str_digits: HashMap<&str, char> = get_str_digits_map();

  let mut min_pos = usize::MAX;
  let mut num = &'0';
  for val in str_digits.values() {
    if let Some(pos) = s.find(*val) {
      if pos <= min_pos {
        min_pos = pos;
        num = val;
      }
    }
  }

  return (*num, min_pos);
}

fn match_last_char_digit(s: &str) -> (char, usize) {
  let str_digits: HashMap<&str, char> = get_str_digits_map();

  let mut max_pos = 0;
  let mut num = &'0';
  for val in str_digits.values() {
    if let Some(pos) = s.rfind(*val) {
      if pos >= max_pos {
        max_pos = pos;
        num = val;
      }
    }
  }

  return (*num, max_pos);
}

fn part2(input: &str) -> u32 {
  let mut sum: u32 = 0;

  for line in input.lines() {
    let mut num = String::with_capacity(2);
    let (first_str_digit, fsd_pos) = match_first_str_digit(line);
    let (last_str_digit, lsd_pos) = match_last_str_digit(line);
    let (first_char_digit, fcd_pos) = match_first_char_digit(line);
    let (last_char_digit, lcd_pos) = match_last_char_digit(line);

    if fsd_pos < fcd_pos && first_str_digit != '0' {
      num.push(first_str_digit)
    } else {
      num.push(first_char_digit)
    }

    if lsd_pos > lcd_pos && last_str_digit != '0' {
      num.push(last_str_digit)
    } else {
      num.push(last_char_digit)
    }

    print!("{} {}\n", line, num);
    // print!(
    //   "{} {} {} {} {}\n",
    //   first_str_digit, last_str_digit, first_char_digit, last_char_digit, num
    // );

    match num.parse::<u32>() {
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
    let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";
    assert_eq!(part2(input), 281);
  }
}
