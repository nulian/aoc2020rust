use super::file_reader;
use regex::Regex;
use std::path::Path;

#[derive(Debug)]
struct PasswordPolicy {
  password: String,
  mandatory_char: String,
  num_a: u8,
  num_b: u8,
}

pub fn run_assignment() {
  let data = load_file_to_vector();
  part1(&data);
  part2(&data);
}

fn part2(data: &Vec<PasswordPolicy>) {
  let mut valid_count = 0;
  for item in data {
    let char_a = fetch_char_from_password(&item.password, (item.num_a - 1) as usize);
    let char_b = fetch_char_from_password(&item.password, (item.num_b - 1) as usize);
    if char_a != char_b && (item.mandatory_char == char_a || item.mandatory_char == char_b) {
      valid_count += 1;
    }
  }
  println!("Valid part2: {:?}", valid_count);
}

fn part1(data: &Vec<PasswordPolicy>) {
  let mut valid_count = 0;
  for item in data {
    let count = item
      .password
      .as_str()
      .matches(item.mandatory_char.as_str())
      .count();
    if count >= item.num_a as usize && count <= item.num_b as usize {
      valid_count += 1;
    }
  }
  println!("Valid part1: {:?}", valid_count);
}

fn fetch_char_from_password(password: &String, index: usize) -> String {
  return password.chars().nth(index).unwrap_or_default().to_string();
}

fn load_file_to_vector() -> Vec<PasswordPolicy> {
  let mut data: Vec<PasswordPolicy> = Vec::new();

  let file_data = file_reader::read_lines(Path::new("./src/input_assignment2.txt"));
  let re = Regex::new(r"^(\d+)-(\d+)\s+(\w+):\s(\w+)$").unwrap();
  match file_data {
    Ok(lines) => {
      for line in lines {
        if let Ok(password_validation) = line {
          let groups = re.captures(&password_validation).unwrap();
          data.push(PasswordPolicy {
            password: String::from(&groups[4]),
            mandatory_char: String::from(&groups[3]),
            num_a: groups[1].parse().unwrap(),
            num_b: groups[2].parse().unwrap(),
          });
        }
      }
    }
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
  return data;
}
