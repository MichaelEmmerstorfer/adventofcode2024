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
    let mut increasing = 0;
    let mut decreasing = 0;
    /* 1 is indicating increasing, 2 indicates decreasing, 0 is inital value  */
    let mut isLevelIncreasing = 0;
    let mut firstFinding = false;
    let mut levelsString = line.split_whitespace();
    let mut compareLevel: u32 = levelsString.next().unwrap().parse().unwrap();
    let numbersInTheLine = levelsString.clone().count();

    for n in 0..numbersInTheLine
    {
      let mut firstFindingResult = false;
      let nextLevel: u32 = levelsString.next().unwrap().parse().unwrap();
      
      /* Decide whether list is increasing or decreasing */
      if compareLevel < nextLevel
      {
        increasing += 1;
      }

      if compareLevel > nextLevel
      {
        decreasing += 1;
      }

      if compareLevel.abs_diff(nextLevel) == 0 || compareLevel.abs_diff(nextLevel) > 3
      {
        if firstFinding == false
        {
          firstFinding = true;
          firstFindingResult = true;
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


    if (safeReport == true) && (decreasing >= numbersInTheLine - 1 || increasing >= numbersInTheLine - 1)
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
        assert_eq!(result, Some(10));
    }
}
