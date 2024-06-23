/*
  Given an array of integers nums and an integer limit, 
  return the size of the longest non-empty subarray such that the absolute difference between any two elements of this subarray is less than or equal to limit.

  Example:
    Input: nums = [8,2,4,7], limit = 4
    Output: 2 
    Explanation: All subarrays are: 
    [8] with maximum absolute diff |8-8| = 0 <= 4.
    [8,2] with maximum absolute diff |8-2| = 6 > 4. 
    [8,2,4] with maximum absolute diff |8-2| = 6 > 4.
    [8,2,4,7] with maximum absolute diff |8-2| = 6 > 4.
    [2] with maximum absolute diff |2-2| = 0 <= 4.
    [2,4] with maximum absolute diff |2-4| = 2 <= 4.
    [2,4,7] with maximum absolute diff |2-7| = 5 > 4.
    [4] with maximum absolute diff |4-4| = 0 <= 4.
    [4,7] with maximum absolute diff |4-7| = 3 <= 4.
    [7] with maximum absolute diff |7-7| = 0 <= 4. 
  Therefore, the size of the longest subarray is 2.
*/
class Solution {
public:
    int longestSubarray(vector<int> &nums, int limit) {
        int n = nums.size();
        if (n == 1) return 1;

        deque<int> minQ, maxQ;
        int i = 0, result = 1;
        for (int j = 0; j < n; ++j) {
            while (!minQ.empty() && nums[j] < nums[minQ.back()]) minQ.pop_back();
            while (!maxQ.empty() && nums[j] > nums[maxQ.back()]) maxQ.pop_back();
            
            minQ.push_back(j);
            maxQ.push_back(j);
            
            while (nums[maxQ.front()] - nums[minQ.front()] > limit) {
                if (minQ.front() == i) minQ.pop_front();
                if (maxQ.front() == i) maxQ.pop_front();
                ++i;
            }
            
            result = max(result, j - i + 1);
        }
        return result;
    }
};

auto init = []() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    cout.tie(0);
    return 'c';
}();
