/*
  Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.
  You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.

  Example:
    Input: nums = [1,2,1,3,2,5]
    Output: [3,5]
    Explanation:  [5, 3] is also a valid answer.
*/
class Solution {
public:
    vector<int> singleNumber(vector<int>& nums) {
        int x = 0;
        vector<int> result(2, 0);

        for (const int& num : nums) x ^= num;

        unsigned int lowestBit = x & -(unsigned int)x;

        for (const int& num : nums) {
            if ((lowestBit & num) == 0) result[0] ^= num;
            else result[1] ^= num;
        }

        return result;
    }
};
