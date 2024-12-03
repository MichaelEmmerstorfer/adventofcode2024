advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
  let mut value: u32 = 0;

  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

  for (_, [data1, data2]) in re.captures_iter(input).map(|c| c.extract()) {
    value += data1.parse::<u32>().unwrap() * data2.parse::<u32>().unwrap();
  }


  return Some(value);
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut value: u32 = 0;
  let mut enabled = true;

  let re = Regex::new(r"(do|don't)\(\)()|mul\((\d+),(\d+)\)").unwrap();

  for (_, [data1 , data2]) in re.captures_iter(input).map(|c| c.extract()) {
    if data1 == "do" {
      enabled = true;
    }
    else if data1 == "don't" {
      enabled = false;
    }
    else if enabled == true
    {
      value += data1.parse::<u32>().unwrap() * data2.parse::<u32>().unwrap();
    }
  }

  return Some(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
