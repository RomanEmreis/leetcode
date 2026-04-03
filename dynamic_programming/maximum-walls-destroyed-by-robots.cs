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
public class Solution {
    public int MaxWalls(int[] robots, int[] distance, int[] walls) {
        int n = robots.Length;
        int[] idx = Enumerable.Range(0, n).OrderBy(i => robots[i]).ToArray();
        
        long[] pos  = new long[n + 2];
        long[] dist = new long[n + 2];
        pos[0]     = long.MinValue / 2; dist[0] = 0;
        pos[n + 1] = long.MaxValue / 2; dist[n + 1] = 0;
        for (int i = 0; i < n; i++) {
            pos[i + 1]  = robots[idx[i]];
            dist[i + 1] = distance[idx[i]];
        }
        
        int[] sw = (int[])walls.Clone();
        Array.Sort(sw);
        
        int CountWalls(long l, long r) {
            if (l > r) return 0;
            return UpperBound(sw, r) - LowerBound(sw, l);
        }
        
        int wallsOnRobots = 0;
        for (int i = 1; i <= n; i++)
            wallsOnRobots += CountWalls(pos[i], pos[i]);
        
        long[] onlyR  = new long[n + 1]; 
        long[] onlyL  = new long[n + 1]; 
        long[] bothRL = new long[n + 1]; 
        
        for (int i = 0; i <= n; i++) {
            long gL = pos[i] + 1;
            long gR = pos[i + 1] - 1;
            if (gL > gR) continue;
            
            long rEnd = Math.Min(pos[i] + dist[i], gR);       
            long lBeg = Math.Max(pos[i+1] - dist[i+1], gL);   
            
            onlyR[i]  = dist[i]      > 0 ? CountWalls(gL,   rEnd) : 0;
            onlyL[i]  = dist[i+1]    > 0 ? CountWalls(lBeg, gR  ) : 0;
            
            if (dist[i] > 0 && dist[i+1] > 0) {
                long uL = Math.Min(gL,   lBeg);  
                long uR = Math.Max(rEnd, gR);    
                
                if (rEnd >= lBeg - 1) {
                    bothRL[i] = CountWalls(gL, gR);
                    bothRL[i] = CountWalls(gL, gR);
                } else {
                    bothRL[i] = CountWalls(gL, rEnd) + CountWalls(lBeg, gR);
                }
            } else {
                bothRL[i] = Math.Max(onlyR[i], onlyL[i]);
            }
        }
               
        long[] dpL2 = new long[n + 2];
        long[] dpR2 = new long[n + 2];
        dpL2[0] = dpR2[0] = 0;
        
        for (int i = 1; i <= n; i++) {
            long robotWall = CountWalls(pos[i], pos[i]);
            
            dpL2[i] = Math.Max(
                dpR2[i - 1] + bothRL[i - 1],
                dpL2[i - 1] + onlyL[i - 1]
            ) + robotWall;
            
            dpR2[i] = Math.Max(
                dpR2[i - 1] + onlyR[i - 1],
                dpL2[i - 1] + 0
            ) + robotWall;
        }
        
        long ans = Math.Max(
            dpR2[n] + onlyR[n],
            dpL2[n] + 0
        );
        
        return (int)ans;
    }
    
    int LowerBound(int[] arr, long t) {
        int lo = 0, hi = arr.Length;
        while (lo < hi) { int m = (lo+hi)/2; if (arr[m] < t) lo=m+1; else hi=m; }
        return lo;
    }
    int UpperBound(int[] arr, long t) {
        int lo = 0, hi = arr.Length;
        while (lo < hi) { int m = (lo+hi)/2; if (arr[m] <= t) lo=m+1; else hi=m; }
        return lo;
    }
}
