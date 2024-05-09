public class Solution {
    public string RemoveKdigits(string num, int k) {
        if (num.Length == k) return "0";

        Stack<char> stack = [];

        foreach (char digit in num) {
            while (stack.Any() && k > 0 && stack.Peek() > digit) {
                stack.Pop();
                --k;
            }
            stack.Push(digit);
        }

        while (k > 0) {
            stack.Pop();
            --k;
        }

        if (stack.Count == 0) return "0";

        StringBuilder sb = new(stack.Count);
        while (stack.TryPop(out char c)) sb.Insert(0, c);

        int start = 0;
        while (sb.Length > start && sb[start] == '0') ++start;
        sb.Remove(0, start);

        return sb.Length == 0 ? "0" : sb.ToString();
    }
}
