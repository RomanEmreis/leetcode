/*
  In the universe Earth C-137, Rick discovered a special form of magnetic force between two balls if they are put in his new invented basket. 
  Rick has n empty baskets, the ith basket is at position[i], Morty has m balls and needs to distribute the balls into the baskets such that the minimum magnetic force between any two balls is maximum.
  Rick stated that magnetic force between two different balls at positions x and y is |x - y|.

  Given the integer array position and the integer m. Return the required force.

Example 1:
  Input: position = [1,2,3,4,7], m = 3
  Output: 3
  Explanation: Distributing the 3 balls into baskets 1, 4 and 7 will make the magnetic force between ball pairs [3, 3, 6]. The minimum magnetic force is 3. We cannot achieve a larger minimum magnetic force than 3.
*/
public class Solution {
    public int MaxDistance(Span<int> position, int m) {
        position.Sort();
        
        int l = 1, r = (position[^1] - position[0]) / (m - 1);
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (CanPlace(ref position, ref mid, ref m)) {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        return r;
    }

    private static bool CanPlace(ref readonly Span<int> position, ref readonly int force, ref readonly int m) {
        int count = 1;
        int last = position[0];
        foreach (int pos in position) {
            if (pos - last >= force) {
                ++count;
                last = pos;
                
                if (count == m) return true;
            }
        }

        return false;
    }
}
