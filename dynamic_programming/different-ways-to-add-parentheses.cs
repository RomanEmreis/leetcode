/*
  Given a string expression of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. You may return the answer in any order.
  The test cases are generated such that the output values fit in a 32-bit integer and the number of different results does not exceed 104.
  
  Example 1:
    Input: expression = "2-1-1"
    Output: [0,2]
    Explanation:
    ((2-1)-1) = 0 
    (2-(1-1)) = 2
*/
public class Solution {
    private Dictionary<(int, int), List<int>> _cache = new();

    public IList<int> DiffWaysToCompute(string expression) {
        List<int> nums = [];
        List<char> ops = [];

        int currentNum = 0;

        for (int i = 0; i < expression.Length; ++i) {
            char c = expression[i];
            if (char.IsDigit(c)) currentNum = currentNum * 10 + (c - '0');
            else {
                nums.Add(currentNum);
                currentNum = 0;
                ops.Add(c);
            }
        }
        nums.Add(currentNum);

        return Calculate(nums, ops, 0, nums.Count - 1);
    }

    private List<int> Calculate(List<int> nums, List<char> ops, int start, int end) {
        if(_cache.TryGetValue((start, end), out var subResult)) return subResult;
        if (start == end) return [nums[start]];

        List<int> result = [];

        for (int i = start; i < end; ++i) {
            var leftList = Calculate(nums, ops, start, i);
            var rightList = Calculate(nums, ops, i + 1, end);

            foreach (var l in leftList) {
                foreach (var r in rightList) {
                    result.Add(DoCalculate(l, r, ops[i]));
                }
            }
        }
        _cache.Add((start, end), result);
        return result;
    }

    private static int DoCalculate(int first, int second, char op) => op switch {
        '-' => first - second,
        '+' => first + second,
        '*' => first * second,
        _ => 0,
    };
}
