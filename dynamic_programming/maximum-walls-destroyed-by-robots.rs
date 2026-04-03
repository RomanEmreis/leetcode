/*
  3661. Maximum Walls Destroyed by Robots
  
  There is an endless straight line populated with some robots and walls. You are given integer arrays robots, distance, and walls:
      robots[i] is the position of the ith robot.
      distance[i] is the maximum distance the ith robot's bullet can travel.
      walls[j] is the position of the jth wall.
  
  Every robot has one bullet that can either fire to the left or the right at most distance[i] meters.
  A bullet destroys every wall in its path that lies within its range. Robots are fixed obstacles: if a bullet hits another robot before reaching a wall, it immediately stops at that robot and cannot continue.
  
  Return the maximum number of unique walls that can be destroyed by the robots.
  
  Notes:
      A wall and a robot may share the same position; the wall can be destroyed by the robot at that position.
      Robots are not destroyed by bullets.
  
  Example 1:
  Input: robots = [4], distance = [3], walls = [1,10]
  Output: 1
  Explanation:
      robots[0] = 4 fires left with distance[0] = 3, covering [1, 4] and destroys walls[0] = 1.
      Thus, the answer is 1.
  
  Example 2:
  Input: robots = [10,2], distance = [5,1], walls = [5,2,7]
  Output: 3
  Explanation:
      robots[0] = 10 fires left with distance[0] = 5, covering [5, 10] and destroys walls[0] = 5 and walls[2] = 7.
      robots[1] = 2 fires left with distance[1] = 1, covering [1, 2] and destroys walls[1] = 2.
      Thus, the answer is 3.
  
  Example 3:
  Input: robots = [1,2], distance = [100,1], walls = [10]
  Output: 0
  Explanation:
  In this example, only robots[0] can reach the wall, but its shot to the right is blocked by robots[1]; thus the answer is 0.
*/
impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let n = robots.len();
        let m = walls.len();

        let mut items: Vec<(i32, i32)> = Vec::with_capacity(n + m + 2);
        items.push((0, 0));
        items.push((i32::MAX, 0));
        items.extend(robots.iter().copied().zip(distance.iter().copied()));
        items.extend(walls.iter().copied().map(|w| (w, i32::MAX)));
        items.sort_unstable();

        let mut walls_on_robots = 0i32;
        let mut write = 0usize;
        let mut prev_robot_pos = i32::MIN;
        for read in 0..items.len() {
            let (x, d) = items[read];
            if d != i32::MAX {
                prev_robot_pos = x;
                items[write] = (x, d);
                write += 1;
            } else if x != prev_robot_pos {
                items[write] = (x, d);
                write += 1;
            } else {
                walls_on_robots += 1;
            }
        }
        items.truncate(write);

        let mut dp_l = 0i32;
        let mut dp_r = 0i32;
        let mut i = 0usize;

        loop {
            let (lx, ld) = items[i];
            if lx == i32::MAX {
                break;
            }

            let gap_start = i + 1;
            let mut gap_end = gap_start;
            while items[gap_end].1 == i32::MAX {
                gap_end += 1;
            }

            let (rx, rd) = items[gap_end];
            let gap = &items[gap_start..gap_end];
            let total = (gap_end - gap_start) as i32;

            let r_reach = lx + ld;
            let lc = gap.partition_point(|&(w, _)| w <= r_reach) as i32;

            let l_reach = rx - rd;
            let rc = total - gap.partition_point(|&(w, _)| w < l_reach) as i32;

            let both = total.min(lc + rc);

            (dp_l, dp_r) = (
                (dp_l + rc).max(dp_r + both),
                dp_l.max(dp_r + lc),
            );

            i = gap_end;
        }

        dp_l.max(dp_r) + walls_on_robots
    }
}
