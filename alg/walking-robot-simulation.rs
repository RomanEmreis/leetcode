/*
  A robot on an infinite XY-plane starts at point (0, 0) facing north. The robot can receive a sequence of these three possible types of commands:
  -2: Turn left 90 degrees.
  -1: Turn right 90 degrees.
  1 <= k <= 9: Move forward k units, one unit at a time.
  Some of the grid squares are obstacles. The ith obstacle is at grid point obstacles[i] = (xi, yi). If the robot runs into an obstacle, then it will instead stay in its current location and move on to the next command.
  
  Return the maximum Euclidean distance that the robot ever gets from the origin squared (i.e. if the distance is 5, return 25).
  
  Note:
  North means +Y direction.
  East means +X direction.
  South means -Y direction.
  West means -X direction.
  There can be obstacle in [0,0].
  
  Example 1:
    Input: commands = [4,-1,3], obstacles = []
    Output: 25
    Explanation: The robot starts at (0, 0):
    1. Move north 4 units to (0, 4).
    2. Turn right.
    3. Move east 3 units to (3, 4).
    The furthest point the robot ever gets from the origin is (3, 4), which squared is 32 + 42 = 25 units away.

  Example 2:
    Input: commands = [4,-1,4,-2,4], obstacles = [[2,4]]
    Output: 65
    Explanation: The robot starts at (0, 0):
    1. Move north 4 units to (0, 4).
    2. Turn right.
    3. Move east 1 unit and get blocked by the obstacle at (2, 4), robot is at (1, 4).
    4. Turn left.
    5. Move north 4 units to (1, 8).
    The furthest point the robot ever gets from the origin is (1, 8), which squared is 12 + 82 = 65 units
*/
use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let directions = [(0,1), (1,0), (0,-1), (-1,0)];

        let mut obstacle_set: HashSet<(i32, i32)> = HashSet::new();
        for obstacle in obstacles {
            obstacle_set.insert((obstacle[0], obstacle[1]));
        }

        let mut pos = (0, 0);
        let mut direction = 0;
        let mut result = 0;

        for command in commands {
            if command == -1 { direction = (direction + 1) % 4 }
            else if command == -2 { direction = (direction + 3) % 4 }
            else {
                for i in 0..command {
                    let next_x = pos.0 + directions[direction].0;
                    let next_y = pos.1 + directions[direction].1;

                    let new_pos = (next_x, next_y);
                    if obstacle_set.contains(&new_pos) {
                        break;
                    }

                    pos = new_pos;

                    let dist = pos.0 * pos.0 + pos.1 * pos.1;
                    if dist > result {
                        result = dist;
                    }
                }
            }
        }

        result
    }
}
