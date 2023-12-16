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
  let mut boxes: Vec<Vec<(&str, usize)>> = vec![Default::default(); 256];
  for step in steps {
    let parts: Vec<&str> = step.split(|c| ['=', '-'].contains(&c)).collect();
    let box_num = hash(parts[0]);
    if step.contains("-") {
      if let Some((index, _)) = boxes[box_num]
        .iter()
        .enumerate()
        .find(|&(_, &lens)| lens.0 == parts[0])
      {
        boxes[box_num].remove(index);
      }
    } else if step.contains("=") {
      if let Some((index, _)) = boxes[box_num]
        .iter()
        .enumerate()
        .find(|&(_, &lens)| lens.0 == parts[0])
      {
        boxes[box_num][index] = (parts[0], parts[1].parse().expect("Number"))
      } else {
        boxes[box_num].push((parts[0], parts[1].parse().expect("Number")))
      }
    }
  }

  let mut ans = 0;
  for (b, items) in boxes.iter().enumerate() {
    for (slot, item) in items.iter().enumerate() {
      ans += (b + 1) * (slot + 1) * item.1;
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 145);
  }
}
