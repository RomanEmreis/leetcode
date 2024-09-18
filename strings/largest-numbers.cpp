/*
  Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.
  
  Since the result may be very large, so you need to return a string instead of an integer.
  
  Example 1:
    Input: nums = [10,2]
    Output: "210"
  
  Example 2:
    Input: nums = [3,30,34,5,9]
    Output: "9534330"
*/
class Solution {
public:
    string largestNumber(vector<int>& nums) {
        vector<string> num_strs(nums.size());
        for (size_t i = 0; i < nums.size(); ++i) {
            num_strs[i] = to_string(nums[i]);
        }

        sort(num_strs.begin(), num_strs.end(), [](const string& a, const string& b) {
            return a + b > b + a;
        });

        if (num_strs[0] == "0") return "0";

        string result;
        for (const string& num_str : num_strs) {
            result += num_str;
        }

        return result;
    }
};
