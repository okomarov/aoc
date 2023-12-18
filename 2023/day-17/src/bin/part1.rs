use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

#[derive(Debug, Clone, Copy)]
struct Node {
  pos: (usize, usize),
  dir: (i32, i32),
  steps: usize,
  loss: usize,
  prev: Key,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Key {
  pos: (usize, usize),
  dir: (i32, i32),
  steps: usize,
}

fn get_loss(board: &Vec<Vec<usize>>, pos: &(usize, usize)) -> Option<usize> {
  board
    .get(pos.0)
    .and_then(|this_row| this_row.get(pos.1))
    .cloned()
}

fn part1(input: &str) -> usize {
  let board: Vec<Vec<usize>> = input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
    })
    .collect();

  let end_pos = (board.len() - 1, board[0].len() - 1);

  let start = Node {
    pos: (0, 0),
    dir: (0, 1),
    loss: 0,
    steps: 0,
    prev: Key {
      pos: (0, 0),
      dir: (0, 0),
      steps: 0,
    },
  };
  let mut seen: HashMap<Key, (usize, Key)> = HashMap::new();
  let mut queue: VecDeque<Node> = VecDeque::from([start]);
  let mut end_node: Node = start.clone();
  let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

  let mut total_loss = std::usize::MAX;

  while let Some(node) = queue.pop_front() {
    // println!("{:?}", node);

    // End
    if node.pos == end_pos {
      if node.loss < total_loss {
        println!("{:?}", node.loss);
        total_loss = node.loss;
        end_node = node;
      }
      continue;
    }

    let key = Key {
      dir: node.dir,
      pos: node.pos,
      steps: node.steps,
    };

    if node.loss + end_pos.0 - node.pos.0 + end_pos.1 - node.pos.1 > total_loss {
      continue;
    }

    if seen.contains_key(&key) {
      let (previous_loss, _) = seen.get(&key).unwrap();
      //  Equality here is important as it seems to get into a loop?
      if *previous_loss <= node.loss {
        continue;
      }
    }

    seen.insert(key, (node.loss, node.prev));

    // Forward
    if node.steps < 3 {
      let new_pos = (
        (node.pos.0 as i32 + node.dir.0) as usize,
        (node.pos.1 as i32 + node.dir.1) as usize,
      );
      match get_loss(&board, &new_pos) {
        Some(loss) => queue.push_back(Node {
          dir: node.dir,
          pos: new_pos,
          loss: node.loss + loss,
          steps: node.steps + 1,
          prev: Key {
            pos: node.pos,
            dir: node.dir,
            steps: node.steps,
          },
        }),
        None => {}
      }
    }

    //  Left and right
    let index = directions
      .iter()
      .position(|&d| d == node.dir)
      .expect("Direction");

    let prev_index = (index + 4 - 1) % 4; // Wrap around for previous index
    let next_index = (index + 1) % 4;

    let left_dir = directions[prev_index];
    let right_dir = directions[next_index];

    let left_pos = (
      (node.pos.0 as i32 + left_dir.0) as usize,
      (node.pos.1 as i32 + left_dir.1) as usize,
    );

    let right_pos = (
      (node.pos.0 as i32 + right_dir.0) as usize,
      (node.pos.1 as i32 + right_dir.1) as usize,
    );

    match get_loss(&board, &left_pos) {
      Some(loss) => queue.push_back(Node {
        dir: left_dir,
        pos: left_pos,
        loss: node.loss + loss,
        steps: 1,
        prev: Key {
          pos: node.pos,
          dir: node.dir,
          steps: node.steps,
        },
      }),
      None => {}
    }
    match get_loss(&board, &right_pos) {
      Some(loss) => queue.push_back(Node {
        dir: right_dir,
        pos: right_pos,
        loss: node.loss + loss,
        steps: 1,
        prev: Key {
          pos: node.pos,
          dir: node.dir,
          steps: node.steps,
        },
      }),
      None => {}
    }
  }
  // print_board(&board, &end_node, &seen);
  total_loss
}

fn print_board(board: &Vec<Vec<usize>>, end_node: &Node, seen: &HashMap<Key, (usize, Key)>) {
  let mut nodes: HashSet<(usize, usize)> = HashSet::from([end_node.pos]);
  let mut current = seen.get(&end_node.prev).unwrap().1;
  while current.pos != (0, 0) {
    nodes.insert(current.pos);
    current = seen.get(&current).unwrap().1;
  }
  nodes.insert(current.pos);

  board.iter().enumerate().for_each(|(row, line)| {
    line.iter().enumerate().for_each(|(col, &c)| {
      if nodes.contains(&(row, col)) {
        print!("#");
      } else {
        print!("{c}");
      }
    });
    print!(" ");
    line.iter().for_each(|&c| {
      print!("{c}");
    });
    println!();
  });
  println!();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 102);
  }
}
