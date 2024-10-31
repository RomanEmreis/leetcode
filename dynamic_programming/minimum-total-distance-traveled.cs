/*
  There are some robots and factories on the X-axis. You are given an integer array robot where robot[i] is the position of the ith robot. You are also given a 2D integer array factory where factory[j] = [positionj, limitj] indicates that positionj is the position of the jth factory and that the jth factory can repair at most limitj robots.
  The positions of each robot are unique. The positions of each factory are also unique. Note that a robot can be in the same position as a factory initially.
  All the robots are initially broken; they keep moving in one direction. The direction could be the negative or the positive direction of the X-axis. When a robot reaches a factory that did not reach its limit, the factory repairs the robot, and it stops moving.
  At any moment, you can set the initial direction of moving for some robot. Your target is to minimize the total distance traveled by all the robots.
  Return the minimum total distance traveled by all the robots. The test cases are generated such that all the robots can be repaired.
  
  Note that
    All robots move at the same speed.
    If two robots move in the same direction, they will never collide.
    If two robots move in opposite directions and they meet at some point, they do not collide. They cross each other.
    If a robot passes by a factory that reached its limits, it crosses it as if it does not exist.
    If the robot moved from a position x to a position y, the distance it moved is |y - x|.
  
  Example 1:
    Input: robot = [0,4,6], factory = [[2,2],[6,2]]
    Output: 4
    Explanation: As shown in the figure:
    - The first robot at position 0 moves in the positive direction. It will be repaired at the first factory.
    - The second robot at position 4 moves in the negative direction. It will be repaired at the first factory.
    - The third robot at position 6 will be repaired at the second factory. It does not need to move.
    The limit of the first factory is 2, and it fixed 2 robots.
    The limit of the second factory is 2, and it fixed 1 robot.
    The total distance is |2 - 0| + |2 - 4| + |6 - 6| = 4. It can be shown that we cannot achieve a better total distance than 4.
*/
public class Solution {
    public long MinimumTotalDistance(IList<int> robot, int[][] factory) {
        var r = robot.ToArray();
        Array.Sort(r);
        Array.Sort(factory, (i, j) => i[0].CompareTo(j[0]));

        long[,] f = new long[r.Length, factory.Length];
        return Dfs(0, 0);

        long Dfs(int i, int j) {
            if (i == r.Length) return 0;
            if (j == factory.Length) return long.MaxValue / 1000;
            if (f[i,j] != 0) return f[i,j];

            long result = Dfs(i, j + 1);
            long t = 0;

            for (int k = 0; k < factory[j][1]; ++k) {
                if (i + k >= r.Length) break;
                
                t += Math.Abs(r[i + k] - factory[j][0]);
                result = Math.Min(result, t + Dfs(i + k + 1, j + 1));
            }

            f[i,j] = result;
            return result;
        }
    }
}
