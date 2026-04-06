/*
  874. Walking Robot Simulation
  
  A robot on an infinite XY-plane starts at point (0, 0) facing north. The robot receives an array of integers commands, which represents a sequence of moves that it needs to execute. There are only three possible types of instructions the robot can receive:
      -2: Turn left 90 degrees.
      -1: Turn right 90 degrees.
      1 <= k <= 9: Move forward k units, one unit at a time.
  
  Some of the grid squares are obstacles. The ith obstacle is at grid point obstacles[i] = (xi, yi). If the robot runs into an obstacle, it will stay in its current location (on the block adjacent to the obstacle) and move onto the next command.
  
  Return the maximum squared Euclidean distance that the robot reaches at any point in its path (i.e. if the distance is 5, return 25).
  
  Note:
      There can be an obstacle at (0, 0). If this happens, the robot will ignore the obstacle until it has moved off the origin. However, it will be unable to return to (0, 0) due to the obstacle.
      North means +Y direction.
      East means +X direction.
      South means -Y direction.
      West means -X direction.
  
   
  
  Example 1:
  Input: commands = [4,-1,3], obstacles = []
  Output: 25
  Explanation:
  The robot starts at (0, 0):
      Move north 4 units to (0, 4).
      Turn right.
      Move east 3 units to (3, 4).
  
  The furthest point the robot ever gets from the origin is (3, 4), which squared is 32 + 42 = 25 units away.
  
  Example 2:
  Input: commands = [4,-1,4,-2,4], obstacles = [[2,4]]
  Output: 65
  Explanation:
  The robot starts at (0, 0):
      Move north 4 units to (0, 4).
      Turn right.
      Move east 1 unit and get blocked by the obstacle at (2, 4), robot is at (1, 4).
      Turn left.
      Move north 4 units to (1, 8).
  
  The furthest point the robot ever gets from the origin is (1, 8), which squared is 12 + 82 = 65 units away.
  
  Example 3:
  Input: commands = [6,-1,-1,6], obstacles = [[0,0]]
  Output: 36
  Explanation:
  The robot starts at (0, 0):
      Move north 6 units to (0, 6).
      Turn right.
      Turn right.
      Move south 5 units and get blocked by the obstacle at (0,0), robot is at (0, 1).
  
  The furthest point the robot ever gets from the origin is (0, 6), which squared is 62 = 36 units away.
*/
use std::collections::HashSet;

const DIRS: [(i32, i32); 4] = [
    (0, 1), (1, 0), (0, -1), (-1, 0)
];

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles = obstacles
            .into_iter()
            .map(|o| (o[0], o[1]))
            .collect::<HashSet<_>>();

        let mut pos = (0, 0);
        let mut res = 0;
        let mut dir = 0;

        commands.into_iter().for_each(|c| {
            match c {
                -1 => dir = (dir + 1) % 4,
                -2 => dir = (dir + 3) % 4,
                _ => {
                    for i in 0..c {
                        let x = pos.0 + DIRS[dir].0;
                        let y = pos.1 + DIRS[dir].1;

                        let new_pos = (x, y);
                        if obstacles.contains(&new_pos) {
                            break;
                        }

                        pos = new_pos;
                        let dist = (pos.0 * pos.0) + (pos.1 * pos.1);
                        res = res.max(dist);
                    }
                }
            }
        });

        res
    }
}
