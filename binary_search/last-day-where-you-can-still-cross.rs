/*
  1970. Last Day Where You Can Still Cross
  
  There is a 1-based binary matrix where 0 represents land and 1 represents water. 
  You are given integers row and col representing the number of rows and columns in the matrix, respectively.
  Initially on day 0, the entire matrix is land. However, each day a new cell becomes flooded with water. 
  You are given a 1-based 2D array cells, where cells[i] = [ri, ci] represents that on the ith day, 
  the cell on the rith row and cith column (1-based coordinates) will be covered with water (i.e., changed to 1).
  You want to find the last day that it is possible to walk from the top to the bottom by only walking on land cells. 
  You can start from any cell in the top row and end at any cell in the bottom row. You can only travel in the four cardinal directions (left, right, up, and down).
  
  Return the last day where it is possible to walk from the top to the bottom by only walking on land cells.
  
  Example 1:
  Input: row = 2, col = 2, cells = [[1,1],[2,1],[1,2],[2,2]]
  Output: 2
  Explanation: The above image depicts how the matrix changes each day starting from day 0.
  The last day where it is possible to cross from top to bottom is on day 2.
  
  Example 2:
  Input: row = 2, col = 2, cells = [[1,1],[1,2],[2,1],[2,2]]
  Output: 1
  Explanation: The above image depicts how the matrix changes each day starting from day 0.
  The last day where it is possible to cross from top to bottom is on day 1.
  
  Example 3:
  Input: row = 3, col = 3, cells = [[1,2],[2,1],[3,3],[2,2],[1,1],[1,3],[2,3],[3,2],[3,1]]
  Output: 3
  Explanation: The above image depicts how the matrix changes each day starting from day 0.
  The last day where it is possible to cross from top to bottom is on day 3.
*/
impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let row = row as usize;
        let col = col as usize;
        let mut l = 1;
        let mut r = cells.len() - 1;
        let mut res = 0;
        while l <= r {
            let day = (l + r) >> 1;
            if can_cross(day, row, col, &cells) {
                res = day;
                l = day + 1;
            } else {
                r = day - 1;
            }
        }
        res as i32
    }
}

const DIRS: [(i32, i32); 4] = [
    (0, 1), (1, 0), (0, -1), (-1, 0)
];

fn can_cross(day: usize, row: usize, col: usize, cells: &[Vec<i32>]) -> bool {
    let mut matrix = vec![vec![0; col]; row];
    for i in 0..day {
        let x = cells[i][0] - 1;
        let y = cells[i][1] - 1;
        matrix[x as usize][y as usize] = 1;
    }

    let mut q = Vec::new();
    for j in 0..col {
        if matrix[0][j] == 0 {
            q.push((0i32, j as i32));
            matrix[0][j] = 1;
        }
    }

    while let Some((i, j)) = q.pop() {
        for (dx, dy) in DIRS {
            let x = i + dx;
            let y = j + dy;

            if x == row as i32 || y == col as i32 || x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if matrix[x][y] == 1 {
                continue;
            }

            if x == row - 1 {
                return true;
            }

            q.push((x as i32, y as i32));
            matrix[x][y] = 1;
        }
    }

    false
}
