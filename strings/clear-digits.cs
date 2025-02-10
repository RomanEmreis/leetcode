/*
  3174. Clear Digits
  
  You are given a string s.
  Your task is to remove all digits by doing this operation repeatedly:
  Delete the first digit and the closest non-digit character to its left.
  Return the resulting string after removing all digits.
*/
public class Solution {
    public string ClearDigits(string s) {
        Stack<char> st = [];
        for (int i = 0; i < s.Length; ++i) {
            if (!char.IsDigit(s[i])) st.Push(s[i]);
            else _ = st.Pop();
        }

        StringBuilder sb = new(st.Count);
        while (st.TryPop(out char ch)) sb.Append(ch);

        for (int i = 0, j = sb.Length - 1; i < j; ++i, --j) {
            (sb[i], sb[j]) = (sb[j], sb[i]);
        }

        return sb.ToString();
    }
}
