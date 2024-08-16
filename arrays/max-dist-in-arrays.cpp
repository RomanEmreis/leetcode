/*
  You are given m arrays, where each array is sorted in ascending order.
  You can pick up two integers from two different arrays (each array picks one) and calculate the distance. We define the distance between two integers a and b to be their absolute difference |a - b|.
  
  Return the maximum distance.

  Example 1:
    Input: arrays = [[1,2,3],[4,5],[1,2,3]]
    Output: 4
    Explanation: One way to reach the maximum distance 4 is to pick 1 in the first or third array and pick 5 in the second array.

  Example 2:
    Input: arrays = [[1],[1]]
    Output: 0
*/
class Solution {
public:
    int maxDistance(vector<vector<int>>& arrays) {
        int result = 0;
        int mn = arrays[0].front(), mx = arrays[0].back();
        for (size_t i = 1; i < arrays.size(); ++i) {
            int curr_mn = arrays[i].front(), curr_mx = arrays[i].back();

            result = max(result, max(abs(curr_mx - mn), abs(mx - curr_mn)));

            mn = min(mn, curr_mn);
            mx = max(mx, curr_mx);
        }
        return result;
    }
};
