
advent_of_code::solution!(1);


pub fn part_one(input: &str) -> Option<u32> {
    let mut leftNumbers: Vec<u32> = Vec::new();
    let mut rightNumbers: Vec<u32> = Vec::new();
    let mut distance: u32 = 0;

    for line in input.lines() {
      let leftNumberString = line.split_at(5).0.parse().unwrap();
      let rightNumberString = line.split_at(8).1.parse().unwrap();

      leftNumbers.push(leftNumberString);
      
      rightNumbers.push(rightNumberString);

    }

    leftNumbers.sort();
    rightNumbers.sort();

    for n in 0..leftNumbers.len() {
      if leftNumbers.get(n) == rightNumbers.get(n)
      {
        distance += 0;
      }
      else if leftNumbers.get(n) > rightNumbers.get(n)
      {
        distance += leftNumbers.get(n).unwrap() - rightNumbers.get(n).unwrap();
      }
      else {
        distance += rightNumbers.get(n).unwrap() - leftNumbers.get(n).unwrap();
      }


    }

    return Some(distance);
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut leftNumbers: Vec<u32> = Vec::new();
  let mut rightNumbers: Vec<u32> = Vec::new();
  let mut similarityScore: u32 = 0;

  for line in input.lines() {
    let leftNumberString = line.split_at(5).0.parse().unwrap();
    let rightNumberString = line.split_at(8).1.parse().unwrap();

    leftNumbers.push(leftNumberString);
    
    rightNumbers.push(rightNumberString);

  }

  for n in 0..leftNumbers.len()
  {
    let numberOfDuplicates: u32 = rightNumbers.iter().filter(|&x| *x == *leftNumbers.get(n).unwrap()).count().try_into().unwrap();

    if numberOfDuplicates != 0
    {
      similarityScore+= numberOfDuplicates * leftNumbers.get(n).unwrap();
    }


  }


  return Some(similarityScore);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
