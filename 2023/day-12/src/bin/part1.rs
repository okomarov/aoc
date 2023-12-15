fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

fn count_arrangements(s: &str, nums: &[usize]) -> usize {
  let s_len = s.len();
  let nums_len = nums.len();
  if s_len == 0 {
    return 1;
  }

  if nums_len == 0 {
    if s.contains("#") {
      return 0;
    } else {
      return 1;
    }
  }

  let mut count = 0;
  let remaining_steps = s_len - nums.iter().sum::<usize>() - (nums_len - 1) + 1;
  for i in 0..remaining_steps {
    let fits_num = s[i..i + nums[0]].chars().all(|c| c == '#' || c == '?');

    if !fits_num || s[0..i].contains("#") {
      continue;
    }
    println!("{:?} {:?} i: {i}", s, nums);
    if i + nums[0] == s_len {
      return count + 1;
    }

    let next_char = s.chars().nth(i + nums[0]);
    if next_char.expect("Char") == '.' || next_char.expect("Char") == '?' {
      count += count_arrangements(&s[i + nums[0] + 1..], &nums[1..])
    }
  }

  count
}

fn part1(input: &str) -> usize {
  let mut ans = 0;
  input.lines().for_each(|line| {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let nums: Vec<usize> = parts[1]
      .split(",")
      .map(|n| n.parse::<usize>().expect("Number"))
      .collect();
    let count = count_arrangements(&parts[0], &nums);
    println!("{:?} {:?} {count}", parts[0], nums);
    ans += count;
  });

  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
.?..??#?##????#...? 4,1";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 22);
  }
}
