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
public class Solution {
    private readonly static Comparer<string> comparer = Comparer<string>.Create((x, y) => {
        string xy = x + y;
        string yx = y + x;
        return yx.CompareTo(xy);
    });
    public string LargestNumber(int[] nums) {
        List<string> numStrs = nums.Select(n => n.ToString()).ToList();

        numStrs.Sort(comparer);

        if (numStrs[0] == "0") return "0";
        return string.Join("", numStrs);
    }
}
