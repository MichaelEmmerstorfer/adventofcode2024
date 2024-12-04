advent_of_code::solution!(4);

pub fn CheckDiagonalRightUp(locationX: usize, locationsForM :&Vec<usize>, locationsForA :&Vec<usize>, locationsForS :&Vec<usize>, sizePerRow: usize) -> bool {
  let mut retValue = false;

  if (locationsForM.contains(&(1 + locationX - (sizePerRow*1)))) 
    && (locationsForA.contains(&(2 + locationX - (sizePerRow*2))))
    && (locationsForS.contains(&(3 + locationX - (sizePerRow*3)))) {
      retValue = true;
    }

      return retValue;
}

pub fn CheckDiagonalRightDown(locationX: usize, locationsForM :&Vec<usize>, locationsForA :&Vec<usize>, locationsForS :&Vec<usize>, sizePerRow: usize) -> bool {
  let mut retValue = false;

  if  (locationsForM.contains(&(1 + locationX + (sizePerRow*1)))) 
    && (locationsForA.contains(&(2 + locationX + (sizePerRow*2))))
    && (locationsForS.contains(&(3 + locationX + (sizePerRow*3)))) {
      retValue = true;
    }

      return retValue;
}

pub fn CheckDiagonalLeftUp(locationX: usize, locationsForM :&Vec<usize>, locationsForA :&Vec<usize>, locationsForS :&Vec<usize>, sizePerRow: usize) -> bool {
  let mut retValue = false;

  if  (locationsForM.contains(&(locationX - 1 - (sizePerRow*1)))) 
    && (locationsForA.contains(&(locationX - 2 - (sizePerRow*2))))
    && (locationsForS.contains(&(locationX - 3 - (sizePerRow*3)))) {
      retValue = true;
    }

      return retValue;
}

pub fn CheckDiagonalLeftDown(locationX: usize, locationsForM :&Vec<usize>, locationsForA :&Vec<usize>, locationsForS :&Vec<usize>, sizePerRow: usize) -> bool {
  let mut retValue = false;

  if  (locationsForM.contains(&(locationX - 1 + (sizePerRow*1)))) 
    && (locationsForA.contains(&(locationX - 2 + (sizePerRow*2))))
    && (locationsForS.contains(&(locationX - 3 + (sizePerRow*3)))) {
      retValue = true;
    }

      return retValue;
}

pub fn CheckStraightDown(locationX: usize, locationsForM :&Vec<usize>, locationsForA :&Vec<usize>, locationsForS :&Vec<usize>, sizePerRow: usize) -> bool {
  let mut retValue = false;

  if  (locationsForM.contains(&(locationX + (sizePerRow*1)))) 
    && (locationsForA.contains(&(locationX + (sizePerRow*2))))
    && (locationsForS.contains(&(locationX + (sizePerRow*3)))) {
      retValue = true;
    }

      return retValue;
}

pub fn CheckStraightUp(locationX: usize, locationsForM :&Vec<usize>, locationsForA :&Vec<usize>, locationsForS :&Vec<usize>, sizePerRow: usize) -> bool {
  let mut retValue = false;

  if  (locationsForM.contains(&(locationX - (sizePerRow*1)))) 
    && (locationsForA.contains(&(locationX - (sizePerRow*2))))
    && (locationsForS.contains(&(locationX - (sizePerRow*3)))) {
      retValue = true;
    }

      return retValue;
}

pub fn CheckStraightLeft(locationX: usize, locationsForM :&Vec<usize>, locationsForA :&Vec<usize>, locationsForS :&Vec<usize>) -> bool {
  let mut retValue = false;

  if  (locationsForM.contains(&(locationX - 1))) 
    && (locationsForA.contains(&(locationX - 2)))
    && (locationsForS.contains(&(locationX - 3))) {
      retValue = true;
    }

      return retValue;
}

pub fn CheckStraightRight(locationX: usize, locationsForM :&Vec<usize>, locationsForA :&Vec<usize>, locationsForS :&Vec<usize>) -> bool {
 let mut retValue = false;

  if  (locationsForM.contains(&(locationX + 1))) 
    && (locationsForA.contains(&(locationX + 2)))
    && (locationsForS.contains(&(locationX + 3))) {
      retValue = true;
    }

      return retValue;
}



pub fn part_one(input: &str) -> Option<u32> {

  let mut NumberOfAppearances = 0;

  let sizePerRow = input.lines().next().unwrap().len();
  let numberOfRows = input.lines().count();
  let mut currentRowInput = 0;
  let mut locationsForX: Vec<usize> = Vec::new();
  let mut locationsForM: Vec<usize> = Vec::new();
  let mut locationsForA: Vec<usize> = Vec::new();
  let mut locationsForS: Vec<usize> = Vec::new();

  for rowInput in input.lines() {

    for index in 0..sizePerRow {
      if rowInput.get(index..index+1).unwrap().contains("X") {
        locationsForX.push(index + (currentRowInput*sizePerRow));
      }
      else if rowInput.get(index..index+1).unwrap().contains("M")
      {
        locationsForM.push(index + (currentRowInput*sizePerRow));
      }
      else if rowInput.get(index..index+1).unwrap().contains("A")
      {
        locationsForA.push(index + (currentRowInput*sizePerRow));
      }
      else if rowInput.get(index..index+1).unwrap().contains("S")
      {
        locationsForS.push(index + (currentRowInput*sizePerRow));
      }
    }

    currentRowInput += 1;
  }

  for locationX in locationsForX {
    let currentRow = locationX / sizePerRow;

    if currentRow <= numberOfRows - 3
    {
      if CheckStraightDown(locationX, &locationsForM, &locationsForA, &locationsForS, sizePerRow)
      {
        NumberOfAppearances+= 1;
      }
    }

    if currentRow >= 3
    {
      if CheckStraightUp(locationX, &locationsForM, &locationsForA, &locationsForS, sizePerRow)
      {
        NumberOfAppearances+= 1;
      }
    }

    if locationX <= ((sizePerRow * currentRow) + (sizePerRow - 4))
    {
      if CheckStraightRight(locationX, &locationsForM, &locationsForA, &locationsForS)
      {
        NumberOfAppearances+= 1;
      }

      if currentRow <= numberOfRows - 3
      {
        if CheckDiagonalRightDown(locationX, &locationsForM, &locationsForA, &locationsForS, sizePerRow)
        {
          NumberOfAppearances+= 1;
        }
      }

      if currentRow >= 3
      {
        if CheckDiagonalRightUp(locationX, &locationsForM, &locationsForA, &locationsForS, sizePerRow)
        {
          NumberOfAppearances+= 1;
        }
      }

    }

    if locationX >= ((sizePerRow * currentRow) + 3){
      if CheckStraightLeft(locationX, &locationsForM, &locationsForA, &locationsForS)
      {
        NumberOfAppearances+= 1;
      }

      if currentRow <= numberOfRows - 3
      {
        if CheckDiagonalLeftDown(locationX, &locationsForM, &locationsForA, &locationsForS, sizePerRow)
        {
          NumberOfAppearances+= 1;
        }
      }

      if currentRow >= 3
      {
        if CheckDiagonalLeftUp(locationX, &locationsForM, &locationsForA, &locationsForS, sizePerRow)
        {
          NumberOfAppearances+= 1;
        }
      }
    }

  }
  

    
    return Some(NumberOfAppearances);
}

pub fn part_two(input: &str) -> Option<u32> {

  let mut NumberOfAppearances = 0;

  let sizePerRow = input.lines().next().unwrap().len();
  let numberOfRows = input.lines().count();
  let mut currentRowInput = 0;
  let mut locationsForM: Vec<usize> = Vec::new();
  let mut locationsForA: Vec<usize> = Vec::new();
  let mut locationsForS: Vec<usize> = Vec::new();

  for rowInput in input.lines() {

    for index in 0..sizePerRow {
      if rowInput.get(index..index+1).unwrap().contains("M")
      {
        locationsForM.push(index + (currentRowInput*sizePerRow));
      }
      else if rowInput.get(index..index+1).unwrap().contains("A")
      {
        locationsForA.push(index + (currentRowInput*sizePerRow));
      }
      else if rowInput.get(index..index+1).unwrap().contains("S")
      {
        locationsForS.push(index + (currentRowInput*sizePerRow));
      }
    }

    currentRowInput += 1;
  }

  for locationA in locationsForA {
    let currentRow = locationA / sizePerRow;
    let mut xAxis = false;
    let mut yAxis = false;

    if (currentRow != 0 && currentRow != numberOfRows) && locationA >= ((sizePerRow * currentRow) + 1) && locationA <= ((sizePerRow * currentRow) + (sizePerRow - 2))
    {
      /* Only do calculations in here */

      /*  xAxis */
      if locationsForM.contains(&(locationA - sizePerRow - 1))
      {
        if locationsForS.contains(&(locationA + sizePerRow + 1))
        {
          xAxis = true;
        }
      }
      else if locationsForS.contains(&(locationA - sizePerRow - 1)) {
        
        if locationsForM.contains(&(locationA + sizePerRow + 1))
        {
          xAxis = true;
        }

      }

      /* yAxis */
      if locationsForM.contains(&(locationA - sizePerRow + 1))
      {
        if locationsForS.contains(&(locationA + sizePerRow - 1))
        {
          yAxis = true;
        }
      }
      else if locationsForS.contains(&(locationA - sizePerRow + 1)) {

        if locationsForM.contains(&(locationA + sizePerRow - 1))
        {
          yAxis = true;
        }
      }

    }


    if xAxis == true && yAxis == true {
      NumberOfAppearances += 1;
    }
  }




  return Some(NumberOfAppearances);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
