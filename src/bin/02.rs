advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
  let mut safeReports = 0;

  for line in input.lines().map(String::from) {
    let mut safeReport = true;
    let mut increasing = false;
    let mut decreasing = false;
    let mut levelsString = line.split_whitespace();
    let mut compareLevel: u32 = levelsString.next().unwrap().parse().unwrap();

    for n in 0..levelsString.clone().count()
    {

      let nextLevel: u32 = levelsString.next().unwrap().parse().unwrap();
      
      /* Decide whether list is increasing or decreasing */
      if compareLevel < nextLevel
      {
        increasing = true;
      }

      if compareLevel > nextLevel
      {
        decreasing = true;
      }

      if (compareLevel.abs_diff(nextLevel) == 0 || compareLevel.abs_diff(nextLevel) > 3) || (decreasing == true && increasing == true)
      {
        safeReport = false;
        break;
      }

      compareLevel = nextLevel;
    }

    if safeReport == true
    {
      safeReports += 1;
    }
  
  }

  return Some(safeReports)
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut safeReports = 0;

  for line in input.lines().map(String::from) {
    let mut safeReport = true;
    let mut increasing = false;
    let mut decreasing = false;
    /* 1 is indicating increasing, 2 indicates decreasing, 0 is inital value  */
    let mut isLevelIncreasing = 0;
    let mut firstFinding = false;
    let mut levelsString = line.split_whitespace();
    let mut compareLevel: u32 = levelsString.next().unwrap().parse().unwrap();

    for n in 0..levelsString.clone().count()
    {
      let mut firstFindingResult = false;
      let nextLevel: u32 = levelsString.next().unwrap().parse().unwrap();
      
      /* Decide whether list is increasing or decreasing */
      if compareLevel < nextLevel
      {
        increasing = true;

        if isLevelIncreasing == 0
        {
          isLevelIncreasing = 1;
        }
      }

      if compareLevel > nextLevel
      {
        decreasing = true;

        if isLevelIncreasing == 0
        {
          isLevelIncreasing = 2;
        }
      }

      if (compareLevel.abs_diff(nextLevel) == 0 || compareLevel.abs_diff(nextLevel) > 3) || (decreasing == true && increasing == true)
      {
        if firstFinding == false
        {
          firstFinding = true;
          firstFindingResult = true;

          if isLevelIncreasing == 1 {
            decreasing = false;
          }
          else if isLevelIncreasing == 2 {
            increasing = false;
          }

        }
        else {
          safeReport = false;
          break;
        }
        
      }

      if firstFindingResult == false
      {
        compareLevel = nextLevel;
      }

    }

    if safeReport == true
    {
      safeReports += 1;
    }
  
  }

  return Some(safeReports)
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
        assert_eq!(result, Some(4));
    }
}
