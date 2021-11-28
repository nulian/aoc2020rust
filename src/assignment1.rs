use super::file_reader;
use std::path::Path;

pub fn run_assignment() {
  let data = load_file_to_vector();
  part1(&data);
  part2(&data);
}

fn part2(data: &Vec<i32>) -> () {
  let mut first_data_set = data.clone();
  'outer: while let Some(item_a) = first_data_set.pop() {
    for item_b in &first_data_set {
      for compare in data {
        if item_a == *compare {
          ()
        } else if *item_b == *compare {
          ()
        } else if (item_a + *item_b + *compare) == 2020 {
          println!("Result: {}", item_a * *item_b * *compare);
          break 'outer;
        }
      }
    }
  }
}

fn part1(data: &Vec<i32>) -> () {
  let mut mut_data = data.clone();
  while let Some(item) = mut_data.pop() {
    for compare in &mut_data {
      match item {
        item if item == *compare => (),
        item if (item + *compare) == 2020 => {
          println!("Result: {}", item * *compare);
          break;
        }
        _ => (),
      }
    }
  }
}

fn load_file_to_vector() -> Vec<i32> {
  let mut data: Vec<i32> = Vec::new();

  let file_data = file_reader::read_lines(Path::new("./src/input_assignment1.txt"));

  match file_data {
    Ok(lines) => {
      for line in lines {
        if let Ok(number) = line {
          data.push(number.parse().unwrap());
        }
      }
    }
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
  return data;
}
