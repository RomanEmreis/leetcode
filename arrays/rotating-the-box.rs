/*
  1861. Rotating the Box
  
  You are given an m x n matrix of characters boxGrid representing a side-view of a box. Each cell of the box is one of the following:
      A stone '#'
      A stationary obstacle '*'
      Empty '.'
  
  The box is rotated 90 degrees clockwise, causing some of the stones to fall due to gravity. Each stone falls down until it lands on an obstacle,
  another stone, or the bottom of the box. Gravity does not affect the obstacles' positions, and the inertia from the box's rotation does not affect the stones' horizontal positions.
  
  It is guaranteed that each stone in boxGrid rests on an obstacle, another stone, or the bottom of the box.
  
  Return an n x m matrix representing the box after the rotation described above.
  
  Example 1:
  Input: boxGrid = [["#",".","#"]]
  Output: [["."],
           ["#"],
           ["#"]]
  
  Example 2:
  Input: boxGrid = [["#",".","*","."],
                ["#","#","*","."]]
  Output: [["#","."],
           ["#","#"],
           ["*","*"],
           [".","."]]
  
  Example 3:
  Input: boxGrid = [["#","#","*",".","*","."],
                ["#","#","#","*",".","."],
                ["#","#","#",".","#","."]]
  Output: [[".","#","#"],
           [".","#","#"],
           ["#","#","*"],
           ["#","*","."],
           ["#",".","*"],
           ["#",".","."]]
*/
impl Solution {
    pub fn rotate_the_box(mut box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let n = box_grid[0].len();

        box_grid.iter_mut().for_each(|c| {
            let mut k = n;
            for i in (0..n).rev() {
                match c[i] {
                    '*' => k = i,
                    '#' => {
                        k -= 1;
                        c[i] = '.';
                        c[k] = '#';
                    },
                    _ => {}
                }
            }
        });

        (0..n)
            .map(|i| box_grid
                .iter()
                .rev()
                .map(move |r| r[i])
                .collect())
            .collect()
    }
}
